# Gaps Notifier API

## Introduction

- Gaps Notifier provides notifications to multiple users around [gaps](https://gaps.heig-vd.ch/)

## [Gaps](https://gaps.heig-vd.ch/)

- Gaps is an Academical Planning System

## Gaps Notifier

- This api is meant to notify users when events happen in gaps

- This api is run by users for users, which means that it can't notify anyone if no data is fed into it

- To automatize this data feeding, the [gaps-cli](https://github.com/heig-lherman/gaps-cli) or the [gaps-scraper](https://github.com/AndreCostaaa/gaps-scraper) can be used

![](./media/schema.png)
_Main communication schema_

## Communication Sequence

### New Event

1. A new event is detected
2. A POST request is done to the REST API
3. The subscribers to this particular event are notified through webhooks

## Discord

- Integration with discord is very easily done since the gaps-notifier-api uses webhooks to notify users of events. Check this stackoverflow [thread](https://stackoverflow.com/questions/75305136/how-to-create-a-function-to-send-messages)

## Code

- The source code is written in [rust](https://www.rust-lang.org/) using the (axum)[https://docs.rs/axum/latest/axum/] framework

## Deploying

### Docker

- A docker image is available [here](https://hub.docker.com/repository/docker/andrecosta222/gaps-notifier-api/general)

### Requirements

- [redis](https://redis.io/)
- Environnement variables:

```env
JWT_SECRET
ADMIN_TOKEN
REDIS_URL
```

Admin token is the static bearer token for admin usage

## API

#### Retrieving an access token

<details>
 <summary><code>POST</code> <code><b>/api/token</b></code> <code>(Retrieves an access token)</code></summary>

##### Parameters

> | name | type     | data type   | description           |
> | ---- | -------- | ----------- | --------------------- |
> | None | required | object JSON | { user_id: <user_id>} |

##### Responses

> | http code | content-type               | response                                 |
> | --------- | -------------------------- | ---------------------------------------- |
> | `201`     | `text/plain;charset=UTF-8` | `Configuration created successfully`     |
> | `400`     | `application/json`         | `{"code":"400","message":"Bad Request"}` |
> | `405`     | `text/html;charset=utf-8`  | None                                     |

##### Example cURL

> ```javascript
> curl -X POST -H "Content-Type: application/json" -d '{"user_id": <user_id>}' http://localhost:3000/api/token
> ```

</details>

---
