use anchor_lang::prelude::*; //Se utiliza el 100% de todos los casos

declare_id!("");

#[programa] //Significa que el código va a ser usado en el programa
pub mod biblioteca{
    use super::*; //Exportar todo el código

    pub fn crear_biblioteca() -> Result<()>{
        //Código...
    }
}

#[account]
#[derive(InitSpace)]
pub struct Biblioteca{
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    libro: Vec<Libro>,
} 

#[derive(AnchorSeriolize, AnchorDeseriolize, Clone, InitSpace, PartialEq, Debug)]
pub struct Libro {
    #[max_len(60)]
    nombre: String,

    paginas: u16,
    
    disponible: bool,
}

#[derive(Account)]
pub struct NuevaBiblioteca {
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

pub struct NuevoLibro {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub biblioteca: Account<'info, Biblioteca>,
}
