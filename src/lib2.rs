use anchor_lang::prelude::*; //Se utiliza el 100% de todos los casos

declare_id!("5bVdMK7qMACeJiAjTxXpgYahdve6YFeR5GdBFNPHVNFd");

#[program] //Significa que el código va a ser usado en el programa
pub mod biblioteca{
    use super::*; //Exportar todo el código

    pub fn crear_biblioteca(context: Context<NuevaBiblioteca>, nombre: String) -> Result<()>{
        let owner_id = context.accounts.owner.key();
        let libros: Vec<Libros> = Vec::new();

        context.accounts.biblioteca.set_inner(Biblioteca {
            owner: owner_id,
            nombre,
            libros,
        });

        Ok(())
    }

    pub fn agregar_libro(context: Context<NuevoLibro>, nombre: String, paginas: u16) -> Result<()>{
        let libro= Libros{
            nombre: nombre,
            paginas,
            disponible: true,
        };

        context.accounts.biblioteca.libros.push(libro);

        Ok(())
    }

    pub fn ver_libros(context: Context<NuevoLibro>) -> Result<()>{
        let libros = context.accounts.biblioteca.libros;
        msg!("La lista de libros es: {:#?}", libros);

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca{
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    libros: Vec<Libros>,
} 

#[derive(AnchorSerialize, AnchorDeseriolize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libros {
    #[max_len(60)]
    nombre: String,

    paginas: u16,
    
    disponible: bool,
}

#[derive(Account)]
pub struct NuevaBiblioteca<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Biblioteca::INIT_SPACE + 8,
        seeds = [b"biblioteca", owner.key().as_ref()]
        bump
    )]
    pub biblioteca: Account<'info, Biblioteca>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)] 
pub struct NuevoLibro<'info> {
    pub owner: Signer<'info>, 
    #[account(mut)] 
    pub biblioteca: Account<'info, Biblioteca>, 

}
