# Rust Canister demo

It's better to have two terminal windows/panels side by side.

On the first terminal:

```
dfx start --clean
dfx deploy
...
2024-11-19 23:12:30.479609 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] New random number generated: 87
2024-11-19 23:12:35.716392 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] New random number generated: 239
...
```

on the second terminal, repeat the following command:

```
dfx canister call demo_backend get_current_number
(171 : nat)
```
