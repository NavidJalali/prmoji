# Prmoji 🍉
A tiny web service that puts emojis on your Slack message when your PR is approved, commented etc. This Rust port was inspired by [this implementation by endreymarcell](https://github.com/endreymarcell/prmoji). 

# How does it work?

If you invite the prmoji bot to your channel, it'll start listening to your messages. Whenever someone posts a GitHub pull request URL, prmoji saves that into the database (URL, message channel, message timestamp).

# Database
You'll need to provide a postgres database. The environment variables for providing access to the database are:
- `DATABASE.HOST` - the host of the database
- `DATABASE.PORT` - the port of the database
- `DATABASE.USER` - the user of the database
- `DATABASE.PASSWORD` - the password of the database
- `DATABASE.DATABASE` - the name of the database
- `DATABASE.POOL_SIZE` - the size of the database connection pool

The schema will be automatically created by the application.

# Validating requests
In order to validate webhook calls by github and slack you need to provide the signing secrets as environment variables.
For slack this can be found in the slack app configuration. For github you will have to create a secret and provide this secret to gihub when setting up a repository webhook for prmoji.

Provide these enviroment variables:
- `SLACK.SIGNING_SECRET`
- `GITHUB.SECRET`

# Sending requests to slack
In order to send requests to slack, you will need to provide a bot token to via the following environment variable. The value can also be found in the slack app configuration.
- `SLACK.BOT_TOKEN`

# Setup


## Slack

- Go to https://api.slack.com/apps/
- Click Your apps
- Click Create New App
- Enter "prmoji" and select your workspace
- On the next page, under Add features and functionality
- Select Event subscriptions
- Click Enable Events
- Add https://{prmoji-url}/slack as the URL
- Navigate to Bot Users
- Click Add a Bot User, then without changing anything click the Add a Bot User below the form
- Navigate back to Event Subscriptions
- Click Enable Events
- Fill out the URL with the same value as above
- Under Subscribe to bot events, select message.channels and message.groups
- Click Install App
- Click Add app to your workspace
- Copy the Bot access token and expose it for the service as described above

## Github

Note: this has to be done for every repository you wish to watch.

- Go to https://github.com/YOUR-USER/YOUR-REPO/settings/hooks
- Click Add webhook
- Add https://{prmoji-url}/github as the URL
- Change the content type to application/json
- Click Let me select individual events
- Tick Issue comments, Pull requests, Pull request reviews, and Pull request review comments
- Click Add webhook
