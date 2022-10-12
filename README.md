# Polkadot Hackathon LATAM 2022

This project took place in the Polkadot Hackathon LATAM 2022 as part of an introduction to substrate workshop.

It consists of an RSVP project in which an user can submit an event and test locally that this event has been created succesfully.

### Build

```bash
cargo build --release
```

### Run

```bash
./target/release/node-template --dev
```

### Test

```bash
cargo test -p pallet-rsvp
```