# Cloudflare DNS updater

## What does it do?
Cloudflare DNS updater updates specified dns records for specified zones effortlessly and automatically. It was originally created to help me hosting my services under dynamic public IP address. Feel free to fork and reuse the code.


## How to use
You can compile the updater from source or download the binary. To compile the program by yourself, you need to have the rustc and cargo installed.

Create your `config.json` that matches the example
```json

{
  "UpdateThreshold": 120,
  "Keys": [
    {
      "AuthKey": "your auth key to some zones",
      "Zones": [
        {
          "ZoneId": "dfhmordntv8t4vb8snittvbies7nct7s47v",
          "Records": ["test2.domain.moe", "test.domain.moe", "tast.domain.moe"]
        }
      ]
    },
    {
      "AuthKey": "your auth key to some other zones",
      "Zones": [
        {
          "ZoneId": "esyigod5tihdvtudn5gitvnecrasdadg",
          "Records": ["error.bababooey.com", "backend.bababooey.com", "bababooey.com"]
        }
      ]
    }
  ]
}

```

### Important
You have to generate your authentication key in cloudflare with necessary permissions to view and edit the specified zones and their dns records, detailed instruction can be found [here](https://support.cloudflare.com/hc/en-us/articles/200167836-Managing-API-Tokens-and-Keys)


## Known bugs
None yet, you tell me
