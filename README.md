# ledmatrix_widgets

A rust application for creating and displaying widgets on the Framework 16 LED Matrix modules.

### Current Widgets
- Current battery life
- CPU usage per-core
- 24hr clock

### Future Additions
- RAM usage
- Disk size
- Network traffic
- Overall CPU usage
- Customize position with application parameters
- Customize refresh rate
- JSON Configuration file

### Installation
This project is now packaged as a Nix flake.

Run the binary directly from the flake:

```bash
nix run .
```

Or install it into your profile:

```bash
nix profile install .
```

### Development

Enter the dev shell with all required build dependencies:

```bash
nix develop
```

Then build or run with Cargo:

```bash
cargo build
cargo run
```

### NixOS Module

This flake exports a NixOS module at `nixosModules.default`
(also `nixosModules.framework-led-matrix`).

Example usage:

```nix
{
  inputs.ledmatrix-widgets.url = "github:superrm11/ledmatrix_widgets";

  outputs = { self, nixpkgs, ledmatrix-widgets, ... }: {
    nixosConfigurations.my-host = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ledmatrix-widgets.nixosModules.default
        {
          programs.framework_led_matrix.enable = true;
        }
      ];
    };
  };
}
```

The module automatically:
- creates a dedicated system user/group for the service
- installs a systemd service (`framework-led-widgets.service`)
- writes `/etc/framework-led-widgets/config.toml`
- installs a udev rule for Framework LED Matrix USB devices

Optional configuration can be provided through
`programs.framework_led_matrix.settings`.

### Home Manager Module

This flake also exports a Home Manager module at `homeManagerModules.default`
(also `homeManagerModules.framework-led-matrix`).

Example usage:

```nix
{
  inputs.ledmatrix-widgets.url = "github:superrm11/ledmatrix_widgets";

  outputs = { self, nixpkgs, home-manager, ledmatrix-widgets, ... }: {
    homeConfigurations.me = home-manager.lib.homeManagerConfiguration {
      pkgs = import nixpkgs { system = "x86_64-linux"; };
      modules = [
        ledmatrix-widgets.homeManagerModules.default
        {
          services.framework-led-widgets.enable = true;
        }
      ];
    };
  };
}
```

Optional configuration can be provided through
`services.framework-led-widgets.settings`.

### Device Access (Home Manager / Manual)

If you run via Home Manager (user service) or manually, your user still needs
access to the serial device (`/dev/ttyACM*` or `/dev/ttyUSB*`).

On NixOS, this usually means adding your user to `dialout`:

```nix
users.users.<your-username>.extraGroups = [ "dialout" ];
```

Check the current owner/group with:

```bash
ls -l /dev/ttyACM* /dev/ttyUSB* 2>/dev/null
```
