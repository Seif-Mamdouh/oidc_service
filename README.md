# Mock Custom OIDC with Github


## TL:DR
- This service serves as a custom validation service for GitHub Actions OIDC tokens
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
- go the github actions tab and generate a JWT token from `Get and Validate JWT` workflow
- you will find a JWT token under artifacts, click to download
- run
  -   ```
      curl -X POST -H "Content-Type: application/json" -d '{"token":"your_generated_token"}' http://localhost:3000/token
      ```
## Expected Ouput:

```
{ sub: "repo:Seif-Mamdouh/oidc_service:ref:refs/heads/main", repository: "Seif-Mamdouh/oidc_service", repository_owner: "Seif-Mamdouh", job_workflow_ref: "Seif-Mamdouh/oidc_service/.github/workflows/test_jwt.yml@refs/heads/main", iat: 1724872875 }
```
