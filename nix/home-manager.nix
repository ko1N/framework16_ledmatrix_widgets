{ self }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.framework-led-widgets;

  configFormat = pkgs.formats.toml { };
  generatedConfig = configFormat.generate "framework-led-widgets-config.toml" cfg.settings;
in
{
  options.services.framework-led-widgets = {
    enable = lib.mkEnableOption "Framework LED Matrix widgets user service";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.system}.framework-led-widgets;
      defaultText = lib.literalExpression "self.packages.${pkgs.system}.framework-led-widgets";
      description = "Package that provides the framework-led-widgets binary.";
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
      description = "TOML settings used by the framework-led-widgets user service.";
      example = {
        general.brightness = 80;
      };
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [ cfg.package ];

    systemd.user.services.framework-led-widgets = {
      Unit = {
        Description = "Framework LED Matrix widgets";
        After = [ "graphical-session.target" ];
        PartOf = [ "graphical-session.target" ];
        "X-Restart-Triggers" = generatedConfig;
      };

      Service = {
        ExecStart = "${lib.getExe cfg.package} --config ${generatedConfig}";
        Restart = "always";
        RestartSec = 5;
      };

      Install = {
        WantedBy = [ "default.target" ];
      };
    };
  };
}
