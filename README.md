###To update the binding and spacetimeDB
- Start spacetimeDB: spacetime start
- Run the following command
spacetime generate --lang typescript --out-dir client/src/module_bindings --project-path server

spacetime publish --project-path server quickstart-chat