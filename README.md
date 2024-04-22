# Finance Tracker

So the idea is basically to create a better finance tracking tool complete with adjustable budgets, spending categories, account tracking, automatic interest accrual, recurring payment reminders, and untracked account adjustments. This is a full stack project, including a user database to store encrypted transaction histories for a user account, sorted by month, a web server to manage saved information, and at least one user client to interact with the backend.

## Potential Technologies

* An SQL platform (e.g. postgres): for the database
* Rust/Rocket: backend server to manage user information
* Typescript/React: frontend web client.
* Rust: desktop client

## Challenges

* Creating a web backend from scratch
* E2E encryption to keep account information safe
  * i.e. how to manage keys and prevent MitM and sniffers
* Deploying a web server
  * It might be possible to do a serverless setup with AWS or something, but that doesn't really gel with a rust backend, which is lame
* Authentication
