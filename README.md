# Cloudflare DNS updater

## What does it do?
Cloudflare DNS updater updates specified dns records for specified zones effortlessly and automatically. It was originally created to help me hosting my services under dynamic public IP address. Feel free to fork and reuse the code.


## How to use
You can compile the updater from source or download the binary. To compile the program by yourself, you need to have the rustc and cargo installed.

Create your `config.json` that matches the example
```json
{
  "AuthKey": "your auth key here",
  "UpdateThreshold": 120,
  "Zones": [
    {
      "ZoneId": "the zone id",
      "Records": [
        "your.dns.com",
        "another.record.moe",
        "example.org"
      ]
    }
  ]
}

```

### Important
You have to generate your authentication key in cloudflare with necessary permissions to view and edit the specified zones and their dns records, detailed instruction can be found [here](https://support.cloudflare.com/hc/en-us/articles/200167836-Managing-API-Tokens-and-Keys)


## Known bugs
None yet, you tell me
