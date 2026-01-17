# user_manager

This repository contains a `user_manager` app hosting simple CRUD APIs to manage a simple users data.
built using Rust Actix Web.

## Running

excute `cargo run` from this `./user_manager/` folder,
the app will initate an sqlite database in this folder (if not initiated yet),
and then it will be running on localhost port 8080 

## API

1. Ping
Check service status, return "ok!" if service is up
```
GET /ping
```

2. List Users
Retrieve list of users recorded in the database `users` table
```
GET /api/users?limit={limit}&offset={offset}=email={email_keyword}
```

3. Get User by Id
Retrieve single user data by its `id` in path param
```
GET /api/users/{id}
```

4. Create User
Create single user, user's `id` is auto-increment
```
POST /api/users

req body:
{
	"name": "string, required",
	"email": "string, required, unique, must follow email format (have '@')"
}
```

5. Update User
Update user's name or email by `id` in path param
```
PUT /api/users/{id}

req body:
{
	"name": "string, required",
	"email": "string, required, unique, must follow email format (have '@')"
}
```

6. Delete User
Hard delete user data from database by `id` in path param
```
DELETE /api/users/{id}
```
