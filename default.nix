{ config, lib, pkgs, ... }:

with lib;

let
  nohostcfg = config.services.nohost;
in
{
  # Service

  options.services.nohost = {
    enable = mkEnableOption "NoHost";
    domain = mkOption {
      type = types.str;
      description = "Domain for nohost to watch";
    };
    bindAddr = mkOption {
      type = types.str;
      default = "0.0.0.0:8080";
      description = "Bind address for NoHost. In the form <IP>:<PORT>.";
    };
    showIP = mkOption {
      type = types.bool;
      default = false;
      description = "Whether NoHost should show types on 404 pages.";
    };
    user = mkOption {
      type = types.str;
      default = "nohost";
      description = "User to run NoHost as";
    };
  };


  # Nohost stuff

  config = mkIf nohostcfg.enable { # Only make the user if it's the default one

    users.users = mkIf (nohostcfg.user == "nohost") {
      nohost = {
        description = "NoHost user";
        isSystemUser = true;
        createHome = false;
      };
    };

    systemd.services.nohost = {
      description = "NoHost service";
      wantedBy = ["multi-user.target"];
      after = [ "network.target" ];
      enable = true;
      environment = {
        NOHOST_DOMAIN = nohostcfg.domain;
        NOHOST_BINDADDR = nohostcfg.bindAddr;
        NOHOST_SHOWIP = if nohostcfg.showIP then "1" else "0";
      };
      serviceConfig = let
        nohostpkg = pkgs.callPackage ./Cargo.nix {};
      in {
        User = nohostcfg.user;
        ExecStart = "${nohostpkg.rootCrate.build}/bin/nohost";
      };
    };

  };


}
