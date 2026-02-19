## Rust Wallet Adapter Templates

### Supported templates

- [x] Dioxus
- [ ] Leptos (under development)
- [x] Yew (legacy deprecated)
- [x] Sycamore (legacy deprecated)

### Maintenance 

New releases will be for bug fixes or to support a new release based on a new version of the supported frameworks.

### Template Features

1. Connect wallet first for routes that require a connected wallet

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/accounts-disconnected.png" alt="Connect Wallet First" width="80%">
2. Connect a wallet

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/connect-modal.png" alt="Connect Modal" width="80%">
3. Connection Loading

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/connect-loading.png" alt="Connect Wallet First" width="80%">
4. Notifications

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/notifications.png" alt="Connect Wallet First" width="50%">
5. Connected Wallet Options

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/dropdown-options.png" alt="Connect Wallet First" width="50%">
6. Homepage

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/Dashboard.png" alt="Connect Wallet First" width="80%">
7. Clusters View

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/clusters-view.png" alt="Connect Wallet First" width="80%">
8. Add clusters view

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/add-cluster-view.png" alt="Connect Wallet First" width="80%">
9. Account Info Loading

   <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/account-loading.png" alt="Connect Wallet First" width="80%">
10. Account Info Loaded (view balance, token accounts and transactions)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/accounts-loaded.png" alt="Connect Wallet First" width="80%">
11. Send SOL (sign and send tx example)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/send.png" alt="Connect Wallet First" width="80%">
12. Receive SOL (with copy address to clipboard support and displaying QR code for receive address)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/receive.png" alt="Connect Wallet First" width="80%">
13. Request Airdrop (not available on Mainnet cluster)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/airdrop.png" alt="Connect Wallet First" width="80%">
14. Failed transaction 

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/failed-tx.png" alt="Connect Wallet First" width="80%">
15. Refresh button to refresh account info after transactions or in case template like Yew returns stale state due to Yew bug
16. Extras route (SIWS, Sign Message, Sign Transaction)

    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/extras.png" alt="Connect Wallet First" width="80%">

17. Unsupported extra features for example wallets that don't support SIWS
    <img src="https://raw.githubusercontent.com/JamiiDao/SolanaWalletAdapter/refs/heads/master/templates/template-screenshots/siws-unsupported.png" alt="Connect Wallet First" width="50%">

 


### To generate the starter code for various templates, first install `cargo generate`

```sh
cargo install cargo-generate

# or if you have `cargo-binstall` already installed, then run:
cargo binstall cargo-generate
```

[Cargo generate](https://crates.io/crates/cargo-generate) is a common rust tool used to generate templates.

- Dioxus template

```sh
cargo generate --name <project name> https://github.com/JamiiDao/SolanaWalletAdapter/ templates/dioxus-adapter
```

- Yew template (deprecated)

```sh
cargo generate --name <project name> https://github.com/JamiiDao/SolanaWalletAdapter/ templates/yew-adapter
```

- Sycamore Template (deprecated)

```sh
cargo generate --name <project name> https://github.com/JamiiDao/SolanaWalletAdapter/ templates/sycamore-adapter
```

##### NOTE: [trunk](https://dioxuslabs.com/learn/0.6/getting_started/) build tool is a tool used to build and bundle Rust code into web-assembly. It has support for tailwind `v4` without the need to install any node modules or the tailwind cli. It achieves this by bundling all the tools required to build and bundle Rust code and tailwind styles to web-assembly.

- Dioxus frontend with Anchor IDL
  ```sh
  cargo generate --name <project name> https://github.com/JamiiDao/SolanaWalletAdapter/ templates/dioxus-adapter-anchor
  ```

- Sycamore frontend with Anchor IDL (deprecated)
  ```sh
  cargo generate --name <project name> https://github.com/JamiiDao/SolanaWalletAdapter/ templates/sycamore-adapter-anchor
  ```

- Yew frontend with Anchor IDL (deprecated)
  ```sh
  cargo generate --name <project name> https://github.com/JamiiDao/SolanaWalletAdapter/ templates/yew-adapter-anchor
  ```

### Running the templates

After generating the template, follow the instructions in the template's `README.md` file in the template root.

Note that for anchor templates, the IDL from the `CARGO_WORKSPACE_DIR/target/{{crate_name}}.json` file using a `.cargo/config.toml` in the root
```toml
[env]
CARGO_WORKSPACE_DIR = { value = "", relative = true }
```

This creates a variable cargo environment variable `CARGO_WORKSPACE_DIR` that makes it easy to locate this JSON IDL file. Remember first to run `anchor build` so that anchor can generate the IDL file.