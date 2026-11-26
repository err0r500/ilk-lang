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
| Actors | member, lending policy (system) · external: library catalog, payment provider |
| Commands | BorrowBook · ReturnBook (implied by "currently have") · IssueLateFine (automatic) · ImportCatalog, RecordFinePayment (from other systems) |
| Events | BookAddedToCatalog, BookBorrowed, BookReturned, FineIssued, FinePaid |
| Views | MyLoans (my current books + due dates) |
| Requires (R3b, R6) | BorrowBook: `isbn` (only catalogued books have one) · ReturnBook: `dueDate` (to compute days overdue) · RecordFinePayment: the fine's `amount` |
| Invariants (R7) | one loan per book · max 3 loans per member · only the borrower returns a book |
| Policies | due = today + 21d · fine = 0.50 × days overdue, none if on time · a payment notification is applied once |
| Streams (R8) | loan: closed by BookReturned · book, member: open-ended |
| Assumptions | the member is the authenticated user · books enter the catalog from a cataloguing system · `BookReturned` records `daysOverdue` |
| Open questions | Can loans be renewed? What happens to unpaid fines (can the member still borrow)? |

## Step 2: Slices and flow graph

| Chapter | Slice | Kind | Reads | Emits / Returns |
|--|--|--|--|--|
| Loans | ImportCatalog | translation | from library catalog | BookAddedToCatalog |
| Loans | BorrowBook | change | BookAddedToCatalog, BookBorrowed, BookReturned | BookBorrowed |
| Loans | ViewMyLoans | view | BookBorrowed, BookReturned | loans list |
| Loans | ReturnBook | change | BookBorrowed, BookReturned | BookReturned |
| Fines | IssueLateFine | automation | trigger BookReturned · FineIssued | FineIssued |
| Fines | RecordFinePayment | translation | from payment provider · FineIssued, FinePaid | FinePaid |

```
importCatalog ──▶ borrowBook ──▶ viewMyLoans ──▶ returnBook ──▶ issueLateFine ──▶ recordFinePayment ─┐
                      ▲              │  ▲            │                                    ▲             │
                      └──────────────┘  └────────────┘                                    └─────────────┘
```

Things the rules forced into the model:
- **R3** made `ReturnBook` necessary: `IssueLateFine` is triggered by `BookReturned`,
  so some slice that precedes it in `flows` must emit it. The same rule required a
  producer for `BookAddedToCatalog` (`ImportCatalog`).
- **R2** lets `ReturnBook` emit `bookId` without asking the client for it: it's an
  attribute of the `BookBorrowed` event the command reads.
- **R6** added the `Err: unknown …` scenarios, and **R8** the `loanIdTag` stream that
  `BookReturned` closes (declared in the board's `closedStreams`).
- The `recordFinePayment → recordFinePayment` self-loop says duplicates can arrive. That's
  what lets the "duplicate notification" scenario's `FinePaid` precondition be reachable.

## Step 3: Files (all pass `ilk check main.ilk`)

`types.ilk` is `assets/types.ilk` copied unchanged.

### tags.ilk
```ilk
import "./types.ilk"

memberIdTag = Tag {memberId Uuid}

bookIdTag = Tag {bookId Uuid}

loanIdTag = Tag {loanId Uuid}
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

libraryCatalog = ExternalSystem {
    name "library catalog"
    description "Cataloguing system where librarians register books"
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
    description "A book became available for lending."

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

    tags [bookIdTag, memberIdTag, loanIdTag]
}

bookReturned = TaggedEvent {
    type "BookReturned"
    description "A member brought a borrowed book back. The loan is over."

    fields {
        loanId Uuid
        bookId Uuid
        memberId Uuid
        returnedOn Date
        daysOverdue Int     // decision-time fact, so the fine automation needs nothing else
    }

    tags [bookIdTag, memberIdTag, loanIdTag]
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

### transImportCatalog.ilk
```ilk
import "./types.ilk"
import "./events.ilk"
import "./tags.ilk"
import "./actors.ilk"

importCatalog = TranslationSlice {
    name "ImportCatalog"
    description "Spec: Members can borrow books from the catalog."

    from libraryCatalog
    channel "topic catalog.books"

    message {
        bookId Uuid
        isbn String
        title String
    }

    command {
        fields {
            bookId Uuid
            isbn String
            title String
        }

        query []

        emits [bookAddedToCatalog]
    }

    requires []

    invariants []

    scenarios []
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

    description "Spec: Members can borrow books from the catalog.
A book can only be lent to one member at a time, and a member can have at most 3 books at once.
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
            {   // the book must exist
                eventTypes [bookAddedToCatalog]
                tags [bookIdTag]
            }
            {   // one loan per book
                eventTypes [bookBorrowed, bookReturned]
                tags [bookIdTag]
            }
            {   // max 3 loans per member
                eventTypes [bookBorrowed, bookReturned]
                tags [memberIdTag]
            }
        ]

        emits [bookBorrowed]
    }

    // only catalogued books have an ISBN
    requires ["isbn"]

    invariants [
        {
            rule "a book is lent to one member at a time"
            violatedBy ["Err: book already on loan"]
        }
        {
            rule "a member has at most 3 books at once"
            violatedBy ["Err: member already has 3 books"]
        }
    ]

    scenarios [
        {
            name "OK: borrow an available book"
            given [bookAddedToCatalog & {fields {bookId "b1"}}]
            when {bookId "b1", memberId "m1"}
            then [bookBorrowed]
        }
        {
            name "OK: a returned book can be borrowed again"
            given [
                bookAddedToCatalog & {fields {bookId "b1"}}
                bookBorrowed & {fields {bookId "b1", memberId "m2"}}
                bookReturned & {fields {bookId "b1", memberId "m2"}}
            ]
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
        {
            name "Err: unknown book"
            given []
            when {bookId "b1"}
            then {error "book not found"}
        }
    ]
}
```

### cmdReturnBook.ilk
```ilk
import "./types.ilk"
import "./events.ilk"
import "./tags.ilk"
import "./actors.ilk"

returnBook = ChangeSlice {
    name "ReturnBook"
    description "Spec: Members can see the books they currently have (which implies they can give them back)."

    actor memberActor

    endpoint {
        method POST
        path "/loans/{loanId}/return"
        params {loanId Uuid}
        auth {userId Uuid}
        responses [{status 204}, {status 404}]
    }

    command {
        fields {
            loanId Uuid
            memberId Uuid = endpoint.auth.userId
            returnedOn Date*
            @doc "days between returnedOn and the loan's dueDate (read from BookBorrowed), 0 if on time"
            daysOverdue Int*
        }

        query [{
            eventTypes [bookBorrowed, bookReturned]
            tags [loanIdTag]
        }]

        // bookId isn't in the request: it comes from the BookBorrowed event read above
        emits [bookReturned]
    }

    // needed to compute daysOverdue
    requires ["dueDate"]

    invariants [{
        rule "only the borrower can return a book"
        violatedBy ["Err: someone else's loan"]
    }]

    scenarios [
        {
            name "OK: return a borrowed book"
            given [bookBorrowed & {fields {loanId "l1", memberId "m1"}}]
            when {loanId "l1", memberId "m1"}
            then [bookReturned]
        }
        {
            name "OK: returning twice changes nothing"
            given [
                bookBorrowed & {fields {loanId "l1", memberId "m1"}}
                bookReturned & {fields {loanId "l1", memberId "m1"}}
            ]
            when {loanId "l1", memberId "m1"}
            then []
        }
        {
            name "Err: someone else's loan"
            given [bookBorrowed & {fields {loanId "l1", memberId "m2"}}]
            when {loanId "l1", memberId "m1"}
            then {error "not your loan"}
        }
        {
            name "Err: unknown loan"
            given []
            when {loanId "l1"}
            then {error "loan not found"}
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

    description "Spec: Members can see the books they currently have and when each is due."

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

    readModel {                 // R1: every field is an attribute of BookBorrowed
        loans [{
            loanId Uuid
            bookId Uuid
            dueDate Date
        }]
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

    requires []

    invariants []

    scenarios [
        {
            name "OK: late return is fined"
            description "4 days late: a fine of 2.00"
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

    // the amount due, set when the fine was issued
    requires ["amount"]

    invariants []

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
import "./transImportCatalog.ilk"
import "./cmdBorrowBook.ilk"
import "./cmdReturnBook.ilk"
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
                description "A member borrows books, tracks and returns them"
                slices [importCatalog, borrowBook, viewMyLoans, returnBook]
            }
            {
                name "Fines"
                description "Late returns are fined and paid online"
                slices [issueLateFine, recordFinePayment]
            }
        ]
    }]

    flows [
        {from importCatalog, to [borrowBook]}
        {from borrowBook, to [viewMyLoans]}
        {from viewMyLoans, to [returnBook, borrowBook]}
        {from returnBook, to [viewMyLoans, issueLateFine]}
        {from issueLateFine, to [recordFinePayment]}
        // the provider may notify the same payment again
        {from recordFinePayment, to [recordFinePayment]}
    ]

    // explicit exceptions to the well-formedness rules
    // the timeline begins with books entering the catalog
    starts [importCatalog]

    unconsumedEvents []

    closedStreams [{stream loanIdTag, closedBy [bookReturned]}]

    openEndedStreams [bookIdTag, memberIdTag]
}
```

## Step 6: Report (shape)

- Files: types, tags, actors, events, transImportCatalog, cmdBorrowBook, cmdReturnBook, viewMyLoans, autoIssueLateFine, transFinePaid, main. `ilk check main.ilk` passes.
- Assumptions: member = authenticated user; books come from a cataloguing system; `BookReturned` records `daysOverdue` so the fine can be computed from the trigger alone.
- Starts: the timeline begins with ImportCatalog (so R3b can prove every borrowed book is in the catalog). Open-ended streams: book, member.
- Open questions: loan renewal? Do unpaid fines block borrowing?
