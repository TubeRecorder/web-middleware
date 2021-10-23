# web-middleware-service

**A RESTful micro service for providing web interface**

---

Provides all the necessary REST APIs to manage the web interface.

## Cloning

To clone this repository:

    git clone --recurse-submodules -j8 git@github.com:TubeRecorder/web-middleware.git

To make sure `master` branch is checked out:

    git submodule foreach "(git checkout master; git pull)&"

## Usage

To start the server:

    cargo run --bin server

## Development

Call the API, the easiest way to do this is to import
the provided [postman collection](postman_collection.json)
into your Postman client.
Note that the command calls return a 204 status with no content.
For feedback on state you should call a query.

## Change Log

A complete history of the change log can be found [here](./ChangeLog.md)

## TODO

An up-to-date list of development aspirations can be found [here](./TODO.md)
