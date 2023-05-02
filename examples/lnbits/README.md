# LNbits Webhook Example

This is an example of running the Lightning Webhook server to receive LNbits webhooks calls.

## Usage

You will need to create invoices using the LNBits API or by using a LNbits extension that supports adding a webhook. You may also want to set a memo or use the `extra` field to assist linking the invoice with a purchase and/or identity.

It's worth noting that LNbits doesn't seem to have any HMAC payload validation for the webhook payload, so you may want to do additional validation to ensure it's valid and accurate.

API Reference: [Payments Create API v1](https://lightning.bitlab.sk/docs#/default/api_payments_create_api_v1_payments_post)

1. Copy `.env.example` to `.env` and update values as needed (`BTCPAY_WEBHOOK_SECRET` can be ignored).
2. Update the `wehbook_handler` function in [main.rs](main.rs) to perform any actions you would like - like updating a database.
3. `cargo run --release --example lnbits`


## Creating Invoices and Testing

You can use the [Demo LNbits](https://legend.lnbits.com/) instance, that uses the production lightning network (real SATs).

Example Invoice Creation with a Webhook URL
```bash
curl -X POST https://legend.lnbits.com/api/v1/payments -d '{"out": false, "amount": 10, "memo": "test webhook", "webhook": "https://MY_WEBHOOK_SERVER/lnbits/webhook"}' -H "X-Api-Key: API_KEY" -H "Content-type: application/json"
```

Example Webhook JSON Response
```JSON
{
  "checking_id": "5d7ee3af76e25677a02ecc132e0e527eab8ffc92b0af2ae5cf16aae7706ecba2",
  "pending": true,
  "amount": 10000,
  "fee": 0,
  "memo": "test webhook",
  "time": 1682947081,
  "bolt11": "lnbc100n1pjyl0qfsp533xd0y5zfakhm0ytyauhlgd0jvh0c03ucw7c7eeynaljgzh6yz9spp5t4lw8tmkuft80gpwesfjurjj064cllyjkzhj4ew0z64wwurwew3qdq5w3jhxapqwajky6r0da4sxqzjccqpjrzjqwz34f2ec60uwx0cfhmvfq9lw4j52ct98jr4p5nqwqluynewq7qkszl3wgqq9jqqqqqqqqqqqqqqqqcqjq9qyysgq70qtyljrcp64m7q8lfzezxp2zfasun9flx7mg6aej262gqxsrw94uj0w2y5664ymkapuwrv0gmdzctrjfx0j3xu9qjyeeze5yw0jkhcpqwasks",
  "preimage": "0000000000000000000000000000000000000000000000000000000000000000",
  "payment_hash": "5d7ee3af76e25677a02ecc132e0e527eab8ffc92b0af2ae5cf16aae7706ecba2",
  "expiry": 1682947681,
  "extra": {},
  "wallet_id": "b59d05b23e184fa69a13a68c29d62df3",
  "webhook": "https://MY_WEBHOOK_SERVER/lnbits/webhook",
  "webhook_status": null
}
```

Example Payment GET (for reference)
```bash
curl -X 'GET' \
  'https://legend.lnbits.com/api/v1/payments/5d7ee3af76e25677a02ecc132e0e527eab8ffc92b0af2ae5cf16aae7706ecba2' \
  -H 'accept: application/json' \
  -H 'X-Api-Key: API_KEY'

```

Example GET Payment Request Response. If you call this in the wehbook_handler function, the data will be similar except that `paid` should be `true`, `pending` should be `false`, `webhook_status` should be `200` if you returned a successful result, and it should include a preimage - provided everything went well.
```JSON
{
    "paid": true,
    "preimage": "0000000000000000000000000000000000000000000000000000000000000000",
    "details":
    {
        "checking_id": "5d7ee3af76e25677a02ecc132e0e527eab8ffc92b0af2ae5cf16aae7706ecba2",
        "pending": false,
        "amount": 10000,
        "fee": 0,
        "memo": "test webhook",
        "time": 1682947081,
        "bolt11": "lnbc100n1pjyl0qfsp533xd0y5zfakhm0ytyauhlgd0jvh0c03ucw7c7eeynaljgzh6yz9spp5t4lw8tmkuft80gpwesfjurjj064cllyjkzhj4ew0z64wwurwew3qdq5w3jhxapqwajky6r0da4sxqzjccqpjrzjqwz34f2ec60uwx0cfhmvfq9lw4j52ct98jr4p5nqwqluynewq7qkszl3wgqq9jqqqqqqqqqqqqqqqqcqjq9qyysgq70qtyljrcp64m7q8lfzezxp2zfasun9flx7mg6aej262gqxsrw94uj0w2y5664ymkapuwrv0gmdzctrjfx0j3xu9qjyeeze5yw0jkhcpqwasks",
        "preimage": "0000000000000000000000000000000000000000000000000000000000000000",
        "payment_hash": "5d7ee3af76e25677a02ecc132e0e527eab8ffc92b0af2ae5cf16aae7706ecba2",
        "expiry": 1682947681.0,
        "extra": {},
        "wallet_id": "b59d05b23e184fa69a13a68c29d62df3",
        "webhook": "https://MY_WEBHOOK_SERVER/lnbits/webhook",
        "webhook_status": 200
    }
}
```
