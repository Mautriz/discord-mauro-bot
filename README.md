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
- Esecuzione singole azioni di gioco
- Check validitá dei singoli comandi dai giocatori
- Aggiungere regole per checkare se il game è finito/qualcuno ha vinto a fine giorno/notte
- Capire come aprire chat private di gruppo
- Cercare di capire come testare questa roba
- Capire come refactorare i vari read nestati dei RwLock perchè c'é un botto di codice duplicato pessimo


##### Specifiche da aggiungere singoli elementi
- Lupi: a fine giornata contare i wolfvote e provare a killare quello con più voti (in caso di pareggio di voti, uno a caso tra quelli nel pareggio)
- Kill: se kill colpisce una persona protetta, non deve ucciderla, se kill colpisce Dorian Grey, deve cercare la persona col quadro e uccidere lui