## MauroBOT

Testing discord bot creation for the first time  
It's purpose is just to understand discords bots functionality, learn rust a little better, and maybe make something useful for me and my friends (?)


### Functionality

- `invite`: create an invite link that works only for 10 minutes and is temporary (when the user logs out, it automatically kicked)
The only functionality atm is create temporary invite links for the discord with the command `invite`
- `random`: create a random int from the user input, takes 2 integers
- `lupus`: a group of commands (wip) to manage a lupus game


### Configuration

It loads env variables from an .env file at the project root  

The file should contain:
- DISCORD_TOKEN (the token in the bot page of the discord developer portal)


### Todo

SUPER MEGA WIPPONE DEVASTANTANTE MORTALE DEVO ANCORA FARE UN BOTTO DI ROBA
- Check di stato di gioco per registrazione comandi (giorno o notte)
- Check funzionamento effettivo del gioco
- Migliorare i messaggi mandati ai giocatori


## Da migliorare
- Ci sono un sacco di unwrap, non dovrebbero esserci (se non quando si può dare per certo che l'unwrap vada a buon fine, ex: guild_id nei comandi solo per canali guild), dovrebbero essere gestiti tutti gli errori
- Tutti questi lock mi hanno reso la vita difficile, sono da ridurre al minimo tutti i tempi di lock (già fixati un paio di deadlock, probabilmente ne troverò altri), ho usato lock al posto di un db/cache tipo redis/pg perché più performante (tanto c'è lo sharding)
- Non ho idea di come  testare questa roba, ne a mano (avendo bisogno di almeno 8 player) ne con test automatici