# Mock Custom OIDC with Github


## TL:DR
- This service serves as a custom validation service for GitHub Actions OIDC tokens
- Allows users to have access control over Github's action infrastructure based on the contents of OIDC tokens.


## Main components
- `GitHubClaims` struct:
  - Defines the structure for GitHub OIDC token claims, including repository, owner, and workflow information.
- `AppState` struct:
  - Holds the shared application state, containing a thread-safe reference to the JWKS (JSON Web Key Set).
- `TokenRequest` struct:
  - Represents the incoming token validation request, containing a single token field.
- `token_endpoint` function:
  - Handles POST requests to /token, validating the provided GitHub token and returning the claims if valid.
- `validate_github_token` function:
  - Performs the actual token validation, including:
    - Checking token format
    - Decoding the token header
    - Finding the appropriate key in the JWKS
    - Decoding and validating the token
    - Checking organization and repository claims against environment variables
- `fetch_jwks` function:
  - Fetches the JWKS from GitHub's OIDC provider URL.
- `hello` function:
  - A simple handler for the root path ("/"), returning "Hello, OIDC!".
- `main` function:
  - Initializes logging and error handling
  - Fetches the initial JWKS
  - Sets up the HTTP server with routes for "/" and "/token"
  - Starts the server on port 3000

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
