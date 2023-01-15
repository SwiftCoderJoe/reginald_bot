# Reggiebot

Reggiebot is a simple Discord bot designed to annoy a friend by replying with random gifs indicating either approval or disapproval of the original message.

For each message of a particular user, Reggiebot has a 1 in 40 chance of replying with a gif. Gifs that indicate approval or disapproval each have a 7 in 15 chance of appearing, with a third, unsure reply having a 1 in 15 chance to appear.

## Building and Deploying

Reggiebot is built with cargo and deployed with Docker. To build the Docker image, run `docker build . --tag CONTAINERNAME`. The Docker image must then be passed a config file to `/config.toml` through a volume or mount, such as in this `docker-compose.yaml` file:

```yaml
version: "3"
services:
  reggiebot:
    image: CONTAINERNAME
    volumes:
      - ./config.toml:/config.toml
    restart: unless-stopped
```

### Configuration File

The config file has four options. First, `token` is a string that represents your Discord bot token. 

`annoyed_person` is the ID of the user whose messages Reggiebot will reply to. This is not a string, so it can be left without quotes.

`true_gif`, `false_gif`, and `perhaps_gif` are all gifs that Reggiebot will reply with. `true_gif` should indicate approval of the original message and has a 7 in 15 chance of appearing. `false_gif` should indicate disapproval of the original message and also has a 7 in 15 chance of appearing. `perhaps_gif` should indicate an undecided or confused opinion of the original message, and has a 1 in 15 chance of appearing.

## License and Contributions

Reggiebot is licensed under the MIT license.

That being said, Reggiebot is a purely hobbist, experimental project. The code may or may not work entirely as expected, and while it is a working example of how to create a Discord bot using Rust, Reggiebot does not necessarily follow all best practices. Contributions might not be accepted simply because I don't have time to review them or I'd rather learn by implementing the feature myself. 