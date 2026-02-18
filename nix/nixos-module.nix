{ self }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.programs.framework_led_matrix;
  serialAccessGroup = "framework-led-matrix";

  configFormat = pkgs.formats.toml { };
  generatedConfig = configFormat.generate "framework-led-widgets-config.toml" cfg.settings;
in
{
  options.programs.framework_led_matrix = {
    enable = lib.mkEnableOption "Framework LED Matrix widgets system integration";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.system}.framework-led-widgets;
      defaultText = lib.literalExpression "self.packages.${pkgs.system}.framework-led-widgets";
      description = "Package that provides the framework-led-widgets binary.";
    };

    user = lib.mkOption {
      type = lib.types.str;
      default = "framework-led-matrix";
      description = "System user that runs the framework-led-widgets service.";
    };

    settings = lib.mkOption {
      type = configFormat.type;
      default = {
        general = {
          brightness = 100;
        };

        widgets = [
          {
            panel = 0;
            x = 0;
            y = 2;
            setup.Cpu = {
              merge_threads = false;
            };
          }
          {
            panel = 0;
            x = 0;
            y = 20;
            setup.Memory = {
              swap = true;
            };
          }
          {
            panel = 0;
            x = 0;
            y = 25;
            setup.Network = {
              devices = [ "wlan0" ];
            };
          }
          {
            panel = 0;
            x = 0;
            y = 30;
            setup.Battery = { };
          }
          {
            panel = 1;
            x = 0;
            y = 2;
            setup.Clock = { };
          }
        ];
      };
      description = "TOML settings consumed from the Nix store.";
      example = {
        general.brightness = 80;
      };
    };
  };

  config = lib.mkIf cfg.enable {
    users.groups.${serialAccessGroup} = { };

    users.users.${cfg.user} = {
      isSystemUser = true;
      group = serialAccessGroup;
      description = "Framework LED Matrix widgets service user";
    };

    services.udev.extraRules = ''
      SUBSYSTEM=="tty", ATTRS{idVendor}=="32ac", ATTRS{idProduct}=="0020", GROUP="framework-led-matrix", MODE="0660", TAG+="uaccess"
    '';

    systemd.services.framework-led-widgets = {
      description = "Framework LED Matrix widgets";
      wantedBy = [ "multi-user.target" ];
      after = [ "systemd-udevd.service" ];
      unitConfig."X-Restart-Triggers" = generatedConfig;

      serviceConfig = {
        Type = "simple";
        ExecStart = "${lib.getExe cfg.package} --config ${generatedConfig}";
        Restart = "always";
        RestartSec = 5;
        User = cfg.user;
        Group = serialAccessGroup;
        SupplementaryGroups = [ serialAccessGroup ];
      };
    };
  };
}
