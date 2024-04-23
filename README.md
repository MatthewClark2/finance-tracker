# Finance Tracker

So the idea is basically to create a better finance tracking tool complete with adjustable budgets, spending categories, account tracking, automatic interest accrual, recurring payment reminders, and untracked account adjustments. This is a full stack project, including a user database to store encrypted transaction histories for a user account, sorted by month, a web server to manage saved information, and at least one user client to interact with the backend.

## Desired Application Features

1. Transaction recording
  a. Date of transaction
  b. Spending category
  c. Purchase location (optional)
  d. Notes
  e. Associated budget (separate from category)
2. On-the-fly adjustments
  a. These should be tracked separately from actual transactions (i.e. not included in totals)
3. Account recording
  a. Balances
  b. Upcoming payments (for CCs, loans, etc.)
4. Automatically apply recurring transactions
5. High-level analysis
  a. Income tracking
  b. Expense tracking
  c. Places where money is being spent
6. Budgets/spending limits
  a. Allow option to roll-over unspent/overspent limits
  b. Allow option to roll-over into separate category
  c. Allow option to transfer between budgets

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

## Milestones

### 1. CLI Tool

Develop a standalone CLI tool. The purpose of this step is to hash out the general structure of the transaction records and learn how to set up and connect to a database with Rust.

It's also during this phase that I'll go about setting up formatting and linting pre-commit hooks for general use in the repository to keep it clean automatically.

#### 1.1 What I'll Learn

1. Rust
2. Postgres
3. JSON (presumably, while testing functionality)
4. An editor (in this case, VSCode)

### 2. API Backend

This is where we get to the meat of the project, and start working on a RESTful API instead of a basic CLI tool. Update the CLI tool to use the API internally.

Initially, this should be implemented as a from scratch multi-threaded web-server [akin to what we see in the Rust book](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html). Once that's in place, we can switch over to using Rocket for a more production-ready service.

Part of this step is determining what should and should not be processed as part of the API for security purposes.

#### 2.1 What I'll Learn

1. TCP/HTTP
2. Sockets
3. Multi-threading
4. User Authentication
5. Rocket (eventually)
6. Encryption

### 3. Web Portal

This is where we can actually make the product usable by the general public by creating a web portal for the application.

This will be a multi-page portal, with the transaction page being a single page application built using React. Other required pages include:

* Login Page
* Contact Page

The main transactions page should feature multiple views of transaction records including:

1. List of open accounts and balances
2. A time-bounded list of transactions
3. List of budgets and remaining balances
4. etc.

#### 3.1 What I'll Learn

1. React
2. HTML5/CSS
3. Typescript
4. Rocket Templates

### And More
