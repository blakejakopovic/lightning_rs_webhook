# BTCPay Webhook Example

This is an example of running the Lightning Webhook server to receive BTCPay webhooks calls.

## Usage

You will need to create a webhook in the BTCPay Admin Panel, and add the webhook secret to the `.env` config file. This is used for HMAC payload validation, before the request is processed (using middleware).

You can then update the `webhook_handler` function to perform your desired actions - like insert or update your database. You may wish to use the invoice `posData` key on creation to assist linking the invoice with a purchase and/or identity.

API Reference: [Webhook_InvoiceSettled](https://docs.btcpayserver.org/API/Greenfield/v1/#operation/Webhook_InvoiceSettled)

1. Copy `.env.example` to `.env` and update values
2. Update the `wehbook_handler` function in [main.rs](main.rs) to perform any actions you would like - like updating a database.
3. `cargo run --release --example btcpay`


## Creating Invoices and Testing

You can use the [Demo BTCPay Testnet](https://testnet.demo.btcpayserver.org) instance, that uses the testnet lightning network (test SATs only).

To setup a webhook, go to `Settings > Webhooks` from the BTCPay admin panel. You don't need to create invoices for testing the webhook, as you can use the build in `Test` functionality. Alternatively, you can create and pay invoices to trigger production-like webhook calls. If you need a lightning testnet wallet, [https://htlc.me/](https://htlc.me/) is dead easy and works well.
