# TODO

1. Setup infra creation via Pulumi (Secrets, Roles + Policies, Resources). Cloud first? Or postpone the deploy configurations ...
2. Setup local secrets configuration via gnu pass (development secrets available only via gpg-key) and cloud sync
3. Decide wich approach for local development (hot reload with Docker seems to slow)
4. backendsouls-json-hal organization and publish (basic implementation of https://datatracker.ietf.org/doc/html/draft-kelly-json-hal-10),
   for now the standard way to present reponses from the server
5. backendsouls-rfc8984 organization and publish (basic implementation of https://datatracker.ietf.org/doc/rfc8984/)
6. Decide about the approach for services and controllers (do we need this? it will add another layer of indirection, maybe stick to functions endpoints)
7. SeaORM or SQLx (to be decided), we need to consider if we are going to stick with relational databases (maybe mongo?)
8. Create a standard way to deploy via (kubernetes? charts? skafold?) ... to be decided
9. What about users? One approach is consider as an app client to be integrated in a bigger environment. So we should implement the Client part from Oath2?
   There are any SCIM rust library to use with the user information? Shoul create one as well with the minimum features?
10. What about iteroperability with olders standards (iCalendar?)

