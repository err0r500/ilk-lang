``` schema

event {
    _: type
}

command {
    fields: {_: type}
    emits: []event
}

```

```user instance
event "hello" {
    id: uuid
    userId: int
}

command "createUser" {
    fields: {
        name: string
        email: string
    }

    emits: [event.hello]
}


```
