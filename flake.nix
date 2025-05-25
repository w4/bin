{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [ cargo rustc rustfmt pre-commit rustPackages.clippy ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };

        nixosModules.default = { config, lib, pkgs, ... }:
          with lib;
          let
            cfg = config.services.paste-bin;
          in
          {
            options.services.paste-bin = {
              enable = mkEnableOption "paste-bin";
              bindAddress = mkOption {
                default = "[::]:8000";
                description = "Address and port to listen on";
                type = types.str;
              };
              maxPasteSize = mkOption {
                default = 32768;
                description = "Max allowed size of an individual paste";
                type = types.int;
              };
              bufferSize = mkOption {
                default = 1000;
                description = "Maximum amount of pastes to store at a time";
                type = types.int;
              };
            };

            config = mkIf cfg.enable {
              systemd.services.bin = {
                enable = true;
                wantedBy = [ "multi-user.target" ];
                wants = [ "network-online.target" ];
                after = [ "network-online.target" ];
                serviceConfig = {
                  Type = "exec";
                  ExecStart = "${self.defaultPackage."${system}"}/bin/bin --buffer-size ${toString cfg.bufferSize} --max-paste-size ${toString cfg.maxPasteSize} ${cfg.bindAddress}";
                  Restart = "on-failure";

                  CapabilityBoundingSet = "";
                  NoNewPrivileges = true;
                  PrivateDevices = true;
                  PrivateTmp = true;
                  PrivateUsers = true;
                  PrivateMounts = true;
                  ProtectHome = true;
                  ProtectClock = true;
                  ProtectProc = "noaccess";
                  ProcSubset = "pid";
                  ProtectKernelLogs = true;
                  ProtectKernelModules = true;
                  ProtectKernelTunables = true;
                  ProtectControlGroups = true;
                  ProtectHostname = true;
                  RestrictSUIDSGID = true;
                  RestrictRealtime = true;
                  RestrictNamespaces = true;
                  LockPersonality = true;
                  RemoveIPC = true;
                  RestrictAddressFamilies = [ "AF_INET" "AF_INET6" ];
                  SystemCallFilter = [ "@system-service" "~@privileged" ];
                };
              };
            };
          };
      });
}
