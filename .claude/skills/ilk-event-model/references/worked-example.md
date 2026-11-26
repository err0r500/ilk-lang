# Worked example: library lending

## Input spec (human-written)

> Members can borrow books from the catalog. A book can only be lent to one
> member at a time, and a member can have at most 3 books at once. Loans last
> 21 days. Members can see the books they currently have and when each is due.

## Step 1: Interpretation table

| Kind | Items |
|--|--|
| Actors | member |
| Commands | BorrowBook |
| Events | BookAddedToCatalog (external to this spec), BookBorrowed, BookReturned |
| Views | MyLoans (my current books + due dates) |
| Rules | book must exist · one loan per book · max 3 loans per member · due = today + 21d |
| Identifiers / tags | bookId (rule: one loan per book), memberId (rule: max 3 per member) |
| Assumptions | the member is the authenticated user; `BookReturned` exists because "currently have" implies returns |
| Open questions | Who adds books to the catalog? How does a return happen? Are late returns penalized? |

## Step 2: Slices

| Chapter | Slice | Kind | Reads | Emits / Returns |
|--|--|--|--|--|
| Loans | BorrowBook | change | BookAddedToCatalog, BookBorrowed, BookReturned | BookBorrowed |
| Loans | ViewMyLoans | view | BookBorrowed, BookReturned | loans list |

(A complete model would also have `ReturnBook` and `AddBookToCatalog`
slices. They're left out here to keep the example short. The report must
list them as missing.)

## Step 3: Files (all pass `ilk check main.ilk`)

`types.ilk` is `assets/types.ilk` copied unchanged.

### tags.ilk
```ilk
import "./types.ilk"

memberIdTag = Tag {memberId Uuid}

bookIdTag = Tag {bookId Uuid}
```

### actors.ilk
```ilk
import "./types.ilk"

memberActor = Actor {
    name "member"
    description "A registered library user"
}
```

### events.ilk
```ilk
import "./types.ilk"
import "./tags.ilk"

bookAddedToCatalog = TaggedEvent {
    type "BookAddedToCatalog"
    description "A librarian made a book available for lending."

    fields {
        bookId Uuid
        isbn String
        title String
    }

    tags [bookIdTag]
}

bookBorrowed = TaggedEvent {
    type "BookBorrowed"
    description "A member borrowed a book; the loan runs until dueDate."

    fields {
        loanId Uuid
        bookId Uuid
        memberId Uuid
        dueDate Date        // decision-time value, captured in the event
    }

    tags [
        bookIdTag
        memberIdTag
    ]
}

bookReturned = TaggedEvent {
    type "BookReturned"
    description "A member brought a borrowed book back."

    fields {
        loanId Uuid
        bookId Uuid
        memberId Uuid
        returnedOn Date
    }

    tags [
        bookIdTag
        memberIdTag
    ]
}
```

### cmdBorrowBook.ilk
```ilk
import "./types.ilk"
import "./events.ilk"
import "./tags.ilk"
import "./actors.ilk"

borrowBook = ChangeSlice {
    name "BorrowBook"

    description "Spec: A member can borrow an available book.
A member can have at most 3 books at a time.
Loans last 21 days."

    actor memberActor

    endpoint {
        method POST
        path "/books/{bookId}/loans"
        params {bookId Uuid}
        auth {userId Uuid}

        responses [
            {status 201, body {loanId Uuid}}
            {status 409}
        ]
    }

    command {
        fields {
            loanId Uuid*                          // generated here, flows into BookBorrowed
            bookId Uuid                           // implicit: endpoint.params.bookId
            memberId Uuid = endpoint.auth.userId  // explicit: names differ
            @doc "today + 21 days"
            dueDate Date*
        }

        query [
            {   // rule: book must exist
                eventTypes [bookAddedToCatalog]
                tags [bookIdTag]
            }
            {   // rule: one loan per book
                eventTypes [bookBorrowed, bookReturned]
                tags [bookIdTag]
            }
            {   // rule: max 3 loans per member
                eventTypes [bookBorrowed, bookReturned]
                tags [memberIdTag]
            }
        ]

        emits [bookBorrowed]
    }

    scenarios [
        {
            name "OK: borrow an available book"
            given [bookAddedToCatalog & {fields {bookId "b1"}}]
            when {bookId "b1", memberId "m1"}
            then [bookBorrowed]
        }
        {
            name "Err: book already on loan"
            given [
                bookAddedToCatalog & {fields {bookId "b1"}}
                bookBorrowed & {fields {bookId "b1", memberId "m2"}}
            ]
            when {bookId "b1", memberId "m1"}
            then {error "book not available"}
        }
        {
            name "Err: unknown book"
            given []
            when {bookId "b1"}
            then {error "book not found"}
        }
        {
            name "Err: member already has 3 books"
            given [
                bookAddedToCatalog & {fields {bookId "b4"}}
                bookBorrowed & {fields {bookId "b1", memberId "m1"}}
                bookBorrowed & {fields {bookId "b2", memberId "m1"}}
                bookBorrowed & {fields {bookId "b3", memberId "m1"}}
            ]
            when {bookId "b4", memberId "m1"}
            then {error "loan limit reached"}
        }
    ]
}
```

### viewMyLoans.ilk
```ilk
import "./types.ilk"
import "./events.ilk"
import "./tags.ilk"
import "./actors.ilk"

viewMyLoans = ViewSlice {
    name "ViewMyLoans"

    description "Spec: A member can see the books they currently have and when each is due."

    actor memberActor

    endpoint {
        method GET
        path "/members/me/loans"
        auth {userId Uuid}
        responses []
    }

    query [
        {
            eventTypes [bookBorrowed, bookReturned]
            tags [memberIdTag & {memberId Uuid = endpoint.auth.userId}]
        }
    ]

    readModel {                 // manual check: loanId, bookId, dueDate ∈ BookBorrowed.fields ✓
        loans []{
            loanId Uuid
            bookId Uuid
            dueDate Date
        }
    }

    scenarios [
        {
            name "Ex: returned loans are hidden"
            given [
                bookBorrowed & {fields {loanId "l1", memberId "m1"}}
                bookBorrowed & {fields {loanId "l2", memberId "m1"}}
                bookReturned & {fields {loanId "l1", memberId "m1"}}
            ]
            query {memberId "m1"}
            expect {loans [{loanId "l2", bookId "b2", dueDate "2026-12-01"}]}
        }
    ]
}
```

### main.ilk
```ilk
import "./types.ilk"
import "./cmdBorrowBook.ilk"
import "./viewMyLoans.ilk"

@main
board = Board {
    name "Library"
    description "Book lending for a public library"
    contexts [{
        name "Lending"
        description "Members borrow and return books"
        chapters [{
            name "Loans"
            description "A member borrows a book and tracks it"
            flow [borrowBook, viewMyLoans]
        }]
    }]
}
```

## Step 6: Report (shape)

- Files: types, tags, actors, events, cmdBorrowBook, viewMyLoans, main. `ilk check main.ilk` passes.
- Missing slices: `AddBookToCatalog` (emits BookAddedToCatalog), `ReturnBook` (emits BookReturned).
- Assumptions: member = authenticated user; returns exist.
- Open questions: penalties for late returns? Can a member renew a loan?
