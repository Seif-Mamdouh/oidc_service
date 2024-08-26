# Mock Custom OIDC with Github


## TL:DR
- This program serves as a custom validation service for GitHub Actions OIDC tokens
- Allows users to have access control over Github's action infrastructure based on the contents of OIDC tokens.


## Main components
- fn `token_endpoint`: `POST` requests to `/token`, extracts the token from the request, and validates it.
- fn `validate_github_token`: does the actual work of validating the token, including checking its format, decoding it, and verifying its signature.
- fn `fetch_jwks`: fetches the JSON Web Key Set (JWKS) from GitHub's OIDC provider, which is used for validating tokens in a production environment.
- `HttpServer` (inside of main fn) : This sets up an Actix web server running on `localhost:3000` with two routes: 
  - a GET "/" for a simple hello message.
  - a POST `/token` for token validation.


 ## Use case Scanrio
 - GitHub Actions workflow would generate an OIDC token. 
 - This token can then be sent to our system for validation. 
 - Our system verifies the token and returns the decoded claims if it's valid.

## Test Plan
- Clone repo
- run `cargo watch -x run`
- run
  -   ```
      curl -X POST -H "Content-Type: application/json" -d '{"token":"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjMiLCJuYW1lIjoiU2VpZiIsImlhdCI6MTUxNjIzOTAyMn0.Gm6ToPj0vZvlrlSVYZuFx0WetYbWlycX0Ia5QxPSoRY"}' http://localhost:3000/token
      ```
## Expected Ouput:

```
{"sub":"123","name":"Seif","iat":1516239022}
```


I got my JWT test token from: https://jwt.io/#debugger-io?token=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjMiLCJuYW1lIjoiU2VpZiIsImlhdCI6MTUxNjIzOTAyMn0.Gm6ToPj0vZvlrlSVYZuFx0WetYbWlycX0Ia5QxPSoRYs
