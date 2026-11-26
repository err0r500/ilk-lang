# Worked example: library lending

## Input spec (human-written)

> Members can borrow books from the catalog. A book can only be lent to one
> member at a time, and a member can have at most 3 books at once. Loans last
> 21 days. Members can see the books they currently have and when each is due.
> Books returned late are fined 0.50 per day overdue. Fines are paid online;
> the payment provider notifies us of each payment.

## Step 1: Interpretation table

| Kind | Items |
|--|--|
| Actors | member, lending policy (system) · external: payment provider |
| Commands | BorrowBook · IssueLateFine (automatic, on BookReturned) · RecordFinePayment (from the provider's webhook) |
| Events | BookAddedToCatalog (external to this spec), BookBorrowed, BookReturned, FineIssued, FinePaid |
| Views | MyLoans (my current books + due dates) |
| Rules | book must exist · one loan per book · max 3 loans per member · due = today + 21d · fine = 0.50 × days overdue, none if on time · a payment notification is applied once |
| Identifiers / tags | bookId (rule: one loan per book), memberId (rule: max 3 per member) |
| Assumptions | the member is the authenticated user; `BookReturned` exists because "currently have" implies returns |
| Open questions | Who adds books to the catalog? How does a return happen? Are late returns penalized? |

## Step 2: Slices

| Chapter | Slice | Kind | Reads | Emits / Returns |
|--|--|--|--|--|
| Loans | BorrowBook | change | BookAddedToCatalog, BookBorrowed, BookReturned | BookBorrowed |
| Loans | ViewMyLoans | view | BookBorrowed, BookReturned | loans list |
| Fines | IssueLateFine | automation | trigger BookReturned · FineIssued | FineIssued |
| Fines | RecordFinePayment | translation | message from payment provider · FineIssued, FinePaid | FinePaid |

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

lendingPolicyActor = Actor {
    name "lending policy"
    description "Background processor applying library policies"
}

paymentProvider = ExternalSystem {
    name "payment provider"
    description "Third-party card payment service"
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
        daysOverdue Int
    }

    tags [
        bookIdTag
        memberIdTag
    ]
}

fineIssued = TaggedEvent {
    type "FineIssued"
    description "A member was charged a fine for returning a book late."

    fields {
        fineId Uuid
        loanId Uuid
        memberId Uuid
        amount Money
    }

    tags [memberIdTag]
}

finePaid = TaggedEvent {
    type "FinePaid"
    description "The payment provider confirmed that a member paid a fine."

    fields {
        fineId Uuid
        memberId Uuid
        amount Money
        paymentRef String
    }

    tags [memberIdTag]
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

### autoIssueLateFine.ilk
```ilk
import "./types.ilk"
import "./events.ilk"
import "./tags.ilk"
import "./actors.ilk"

issueLateFine = AutomationSlice {
    name "IssueLateFine"
    description "Spec: Books returned late are fined 0.50 per day overdue."

    actor lendingPolicyActor

    trigger bookReturned

    command {
        fields {
            fineId Uuid*
            loanId Uuid
            memberId Uuid
            daysOverdue Int
            @doc "daysOverdue * 0.50"
            amount Money = compute(trigger.fields.daysOverdue)
        }

        query [{
            eventTypes [fineIssued]
            tags [memberIdTag]
        }]

        emits [fineIssued]
    }

    scenarios [
        {
            name "OK: late return is fined"
            given []
            when bookReturned & {fields {loanId "l1", daysOverdue 4}}
            then [fineIssued]
        }
        {
            name "OK: on-time return is not fined"
            given []
            when bookReturned & {fields {daysOverdue 0}}
            then []
        }
    ]
}
```

### transFinePaid.ilk
```ilk
import "./types.ilk"
import "./events.ilk"
import "./tags.ilk"
import "./actors.ilk"

recordFinePayment = TranslationSlice {
    name "RecordFinePayment"
    description "Spec: Fines are paid online; the payment provider notifies us of each payment."

    from paymentProvider
    channel "webhook POST /webhooks/payments"

    message {
        reference String
        metadata {fineId Uuid, memberId Uuid}
        amountPaid Money
    }

    command {
        fields {
            fineId Uuid = message.metadata.fineId
            memberId Uuid = message.metadata.memberId
            amount Money = message.amountPaid
            paymentRef String = message.reference
        }

        query [{
            eventTypes [fineIssued, finePaid]
            tags [memberIdTag]
        }]

        emits [finePaid]
    }

    scenarios [
        {
            name "OK: payment for an open fine"
            given [fineIssued & {fields {fineId "f1"}}]
            when {reference "pay_123", amountPaid 2.0}
            then [finePaid]
        }
        {
            name "OK: duplicate notification is ignored"
            given [
                fineIssued & {fields {fineId "f1"}}
                finePaid & {fields {fineId "f1"}}
            ]
            when {reference "pay_123"}
            then []
        }
        {
            name "Err: unknown fine"
            given []
            when {reference "pay_999"}
            then {error "fine not found"}
        }
    ]
}
```

### main.ilk
```ilk
import "./types.ilk"
import "./cmdBorrowBook.ilk"
import "./viewMyLoans.ilk"
import "./autoIssueLateFine.ilk"
import "./transFinePaid.ilk"

@main
board = Board {
    name "Library"
    description "Book lending for a public library"
    contexts [{
        name "Lending"
        description "Members borrow and return books"
        chapters [
            {
                name "Loans"
                description "A member borrows a book and tracks it"
                flow [borrowBook, viewMyLoans]
            }
            {
                name "Fines"
                description "Late returns are fined and paid online"
                flow [issueLateFine, recordFinePayment]
            }
        ]
    }]
}
```

## Step 6: Report (shape)

- Files: types, tags, actors, events, cmdBorrowBook, viewMyLoans, autoIssueLateFine, transFinePaid, main. `ilk check main.ilk` passes.
- Missing slices: `AddBookToCatalog` (emits BookAddedToCatalog), `ReturnBook` (emits BookReturned).
- Assumptions: member = authenticated user; returns exist; `BookReturned` carries `daysOverdue`, a decision-time fact recorded at return, so the fine can be computed from the trigger alone.
- Open questions: penalties for late returns? Can a member renew a loan?
