{
  inputs.horizon-cli.url = "github:criome/horizon-cli/testing";
  inputs.system.url = "github:criome/system";

  outputs =
    {
      self,
      horizon-cli,
      system,
    }:
    {
      devShells.${system.value}.default = horizon-cli.devShell;
    };
}
