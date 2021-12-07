# Cross language DTO sharing

Just an idea I had to share contracts defined in .NET with other languages, rust in this case. Quite similar to how OpenAPI/Swagger generated clients work.

Building the rust project with cargo will:

1. Build the .NET contracts generator project (which produces the JSON schema files from the contracts project)
2. Link the generated JSON schema files into its own local copy
3. Generate a rust struct using schemafy
4. Get some user input, and then print out the JSON of the hydrated DTO