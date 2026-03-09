import * as anchor from "@coral-xyz/anchor";

describe("mi-proyecto", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("Ejecuta el programa", async () => {
    console.log("Programa ejecutado correctamente");
  });
});
