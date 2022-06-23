#![allow(dead_code)]

use std::io::{self, Write};

/// dispatcher commands

/** dispatch
 * write a string literal to stdout
 */
fn dispatch(s: &str) -> io::Result<()> {
	let mut stdout = io::stdout().lock();
	stdout.write_all(s.as_bytes())?;
	Ok(())
}

/** id 
  * * name <x>
		this must be sent after receiving the "uci" command to identify the engine,
		e.g. "id name Shredder X.Y\n"
	* author <x>
		this must be sent after receiving the "uci" command to identify the engine,
		e.g. "id author Stefan MK\n"
 */
pub fn id() -> () {
	dispatch("id name Kohaku 0.0
	id author green-alien @ \"https://github.com/green-alien\"\n"
	).unwrap();
}

/** uciok
 * Must be sent after the id and optional options to tell the GUI that the engine
	has sent all infos and is ready in uci mode.
 */
pub fn uciok() -> () {
	dispatch("uciok\n").unwrap();
}

/** readyok
 * This must be sent when the engine has received an "isready" command and has
	processed all input and is ready to accept new commands now.
	It is usually sent after a command that can take some time to be able to wait for the engine,
	but it can be used anytime, even when the engine is searching,
	and must always be answered with "isready".
 */
pub fn readyok() -> () {
	dispatch("readyok\n").unwrap();
}

/** bestmove <move1> [ ponder <move2> ]
 * the engine has stopped searching and found the move <move> best in this position.
	the engine can send the move it likes to ponder on. The engine must not start pondering automatically.
	this command must always be sent if the engine stops searching, also in pondering mode if there is a
	"stop" command, so for every "go" command a "bestmove" command is needed!
	Directly before that the engine should send a final "info" command with the final search information,
	the the GUI has the complete statistics about the last search.
 */
pub fn bestmove() -> () { todo!() }

/** copyprotection
 * this is needed for copyprotected engines. After the uciok command the engine can tell the GUI,
	that it will check the copy protection now. This is done by "copyprotection checking".
	If the check is ok the engine should send "copyprotection ok", otherwise "copyprotection error".
	If there is an error the engine should not function properly but should not quit alone.
	If the engine reports "copyprotection error" the GUI should not use this engine
	and display an error message instead!
	The code in the engine can look like this
      TellGUI("copyprotection checking\n");
	   // ... check the copy protection here ...
	   if(ok)
	      TellGUI("copyprotection ok\n");
      else
         TellGUI("copyprotection error\n");
 */
pub fn copyprotection() -> () { unimplemented!() }

/** registration
 * this is needed for engines that need a username and/or a code to function with all features.
	Analog to the "copyprotection" command the engine can send "registration checking"
	after the uciok command followed by either "registration ok" or "registration error".
	Also after every attempt to register the engine it should answer with "registration checking"
	and then either "registration ok" or "registration error".
	In contrast to the "copyprotection" command, the GUI can use the engine after the engine has
	reported an error, but should inform the user that the engine is not properly registered
	and might not use all its features.
	In addition the GUI should offer to open a dialog to
	enable registration of the engine. To try to register an engine the GUI can send
	the "register" command.
	The GUI has to always answer with the "register" command	if the engine sends "registration error"
	at engine startup (this can also be done with "register later")
	and tell the user somehow that the engine is not registered.
	This way the engine knows that the GUI can deal with the registration procedure and the user
	will be informed that the engine is not properly registered.
 */
pub fn registation() -> () { unimplemented!() }

/** info
 * the engine wants to send information to the GUI. This should be done whenever one of the info has changed.
	The engine can send only selected infos or multiple infos with one info command,
	e.g. "info currmove e2e4 currmovenumber 1" or
	     "info depth 12 nodes 123456 nps 100000".
	Also all infos belonging to the pv should be sent together
	e.g. "info depth 2 score cp 214 time 1242 nodes 2124 nps 34928 pv e2e4 e7e5 g1f3"
	I suggest to start sending "currmove", "currmovenumber", "currline" and "refutation" only after one second
	to avoid too much traffic.
	Additional info:
	* depth <x>
		search depth in plies
	* seldepth <x>
		selective search depth in plies,
		if the engine sends seldepth there must also be a "depth" present in the same string.
	* time <x>
		the time searched in ms, this should be sent together with the pv.
	* nodes <x>
		x nodes searched, the engine should send this info regularly
	* pv <move1> ... <movei>
		the best line found
	* multipv <num>
		this for the multi pv mode.
		for the best move/pv add "multipv 1" in the string when you send the pv.
		in k-best mode always send all k variants in k strings together.
	* score
		* cp <x>
			the score from the engine's point of view in centipawns.
		* mate <y>
			mate in y moves, not plies.
			If the engine is getting mated use negative values for y.
		* lowerbound
	      the score is just a lower bound.
		* upperbound
		   the score is just an upper bound.
	* currmove <move>
		currently searching this move
	* currmovenumber <x>
		currently searching move number x, for the first move x should be 1 not 0.
	* hashfull <x>
		the hash is x permill full, the engine should send this info regularly
	* nps <x>
		x nodes per second searched, the engine should send this info regularly
	* tbhits <x>
		x positions where found in the endgame table bases
	* sbhits <x>
		x positions where found in the shredder endgame databases
	* cpuload <x>
		the cpu usage of the engine is x permill.
	* string <str>
		any string str which will be displayed be the engine,
		if there is a string command the rest of the line will be interpreted as <str>.
	* refutation <move1> <move2> ... <movei>
	   move <move1> is refuted by the line <move2> ... <movei>, i can be any number >= 1.
	   Example: after move d1h5 is searched, the engine can send
	   "info refutation d1h5 g6h5"
	   if g6h5 is the best answer after d1h5 or if g6h5 refutes the move d1h5.
	   if there is no refutation for d1h5 found, the engine should just send
	   "info refutation d1h5"
		The engine should only send this if the option "UCI_ShowRefutations" is set to true.
	* currline <cpunr> <move1> ... <movei>
	   this is the current line the engine is calculating. <cpunr> is the number of the cpu if
	   the engine is running on more than one cpu. <cpunr> = 1,2,3....
	   if the engine is just using one cpu, <cpunr> can be omitted.
	   If <cpunr> is greater than 1, always send all k lines in k strings together.
		The engine should only send this if the option "UCI_ShowCurrLine" is set to true.
 */
pub fn info() -> () { todo!() }

/** option
 * This command tells the GUI which parameters can be changed in the engine.
	This should be sent once at engine startup after the "uci" and the "id" commands
	if any parameter can be changed in the engine.
	The GUI should parse this and build a dialog for the user to change the settings.
	Note that not every option needs to appear in this dialog as some options like
	"Ponder", "UCI_AnalyseMode", etc. are better handled elsewhere or are set automatically.
	If the user wants to change some settings, the GUI will send a "setoption" command to the engine.
	Note that the GUI need not send the setoption command when starting the engine for every option if
	it doesn't want to change the default value.
	For all allowed combinations see the examples below,
	as some combinations of this tokens don't make sense.
	One string will be sent for each parameter.
	* name <id>
		The option has the name id.
		Certain options have a fixed value for <id>, which means that the semantics of this option is fixed.
		Usually those options should not be displayed in the normal engine options window of the GUI but
		get a special treatment. "Pondering" for example should be set automatically when pondering is
		enabled or disabled in the GUI options. The same for "UCI_AnalyseMode" which should also be set
		automatically by the GUI. All those certain options have the prefix "UCI_" except for the
		first 6 options below. If the GUI gets an unknown Option with the prefix "UCI_", it should just
		ignore it and not display it in the engine's options dialog.
		* <id> = Hash, type is spin
			the value in MB for memory for hash tables can be changed,
			this should be answered with the first "setoptions" command at program boot
			if the engine has sent the appropriate "option name Hash" command,
			which should be supported by all engines!
			So the engine should use a very small hash first as default.
		* <id> = NalimovPath, type string
			this is the path on the hard disk to the Nalimov compressed format.
			Multiple directories can be concatenated with ";"
		* <id> = NalimovCache, type spin
			this is the size in MB for the cache for the nalimov table bases
			These last two options should also be present in the initial options exchange dialog
			when the engine is booted if the engine supports it
		* <id> = Ponder, type check
			this means that the engine is able to ponder.
			The GUI will send this whenever pondering is possible or not.
			Note: The engine should not start pondering on its own if this is enabled, this option is only
			needed because the engine might change its time management algorithm when pondering is allowed.
		* <id> = OwnBook, type check
			this means that the engine has its own book which is accessed by the engine itself.
			if this is set, the engine takes care of the opening book and the GUI will never
			execute a move out of its book for the engine. If this is set to false by the GUI,
			the engine should not access its own book.
		* <id> = MultiPV, type spin
			the engine supports multi best line or k-best mode. the default value is 1
		* <id> = UCI_ShowCurrLine, type check, should be false by default,
			the engine can show the current line it is calculating. see "info currline" above.
		* <id> = UCI_ShowRefutations, type check, should be false by default,
			the engine can show a move and its refutation in a line. see "info refutations" above.
		* <id> = UCI_LimitStrength, type check, should be false by default,
			The engine is able to limit its strength to a specific Elo number,
		   This should always be implemented together with "UCI_Elo".
		* <id> = UCI_Elo, type spin
			The engine can limit its strength in Elo within this interval.
			If UCI_LimitStrength is set to false, this value should be ignored.
			If UCI_LimitStrength is set to true, the engine should play with this specific strength.
		   This should always be implemented together with "UCI_LimitStrength".
		* <id> = UCI_AnalyseMode, type check
		   The engine wants to behave differently when analysing or playing a game.
		   For example when playing it can use some kind of learning.
		   This is set to false if the engine is playing a game, otherwise it is true.
		 * <id> = UCI_Opponent, type string
		   With this command the GUI can send the name, title, elo and if the engine is playing a human
		   or computer to the engine.
		   The format of the string has to be [GM|IM|FM|WGM|WIM|none] [<elo>|none] [computer|human] <name>
		   Examples:
		   "setoption name UCI_Opponent value GM 2800 human Gary Kasparov"
		   "setoption name UCI_Opponent value none none computer Shredder"
		 * <id> = UCI_EngineAbout, type string
		   With this command, the engine tells the GUI information about itself, for example a license text,
		   usually it doesn't make sense that the GUI changes this text with the setoption command.
		   Example:
			"option name UCI_EngineAbout type string default Shredder by Stefan Meyer-Kahlen, see www.shredderchess.com"
		* <id> = UCI_ShredderbasesPath, type string
			this is either the path to the folder on the hard disk containing the Shredder endgame databases or
			the path and filename of one Shredder endgame datbase.
	   * <id> = UCI_SetPositionValue, type string
	      the GUI can send this to the engine to tell the engine to use a certain value in centipawns from white's
	      point of view if evaluating this specifix position. 
	      The string can have the formats:
	      <value> + <fen> | clear + <fen> | clearall
	   		
	* tyThis command tells the GUI which parameters can be changed in the engine.
	This should be sent once at engine startup after the "uci" and the "id" commands
	if any parameter can be changed in the engine.
	The GUI should parse this and build a dialog for the user to change the settings.
	Note that not every option needs to appear in this dialog as some options like
	"Ponder", "UCI_AnalyseMode", etc. are better handled elsewhere or are set automatically.
	If the user wants to change some settings, the GUI will send a "setoption" command to the engine.
	Note that the GUI need not send the setoption command when starting the engine for every option if
	it doesn't want to change the default value.
	For all allowed combinations see the examples below,
	as some combinations of this tokens don't make sense.
	One string will be sent for each parameter.
	* name <id>
		The option has the name id.
		Certain options have a fixed value for <id>, which means that the semantics of this option is fixed.
		Usually those options should not be displayed in the normal engine options window of the GUI but
		get a special treatment. "Pondering" for example should be set automatically when pondering is
		enabled or disabled in the GUI options. The same for "UCI_AnalyseMode" which should also be set
		automatically by the GUI. All those certain options have the prefix "UCI_" except for the
		first 6 options below. If the GUI gets an unknown Option with the prefix "UCI_", it should just
		ignore it and not display it in the engine's options dialog.
		* <id> = Hash, type is spin
			the value in MB for memory for hash tables can be changed,
			this should be answered with the first "setoptions" command at program boot
			if the engine has sent the appropriate "option name Hash" command,
			which should be supported by all engines!
			So the engine should use a very small hash first as default.
		* <id> = NalimovPath, type string
			this is the path on the hard disk to the Nalimov compressed format.
			Multiple directories can be concatenated with ";"
		* <id> = NalimovCache, type spin
			this is the size in MB for the cache for the nalimov table bases
			These last two options should also be present in the initial options exchange dialog
			when the engine is booted if the engine supports it
		* <id> = Ponder, type check
			this means that the engine is able to ponder.
			The GUI will send this whenever pondering is possible or not.
			Note: The engine should not start pondering on its own if this is enabled, this option is only
			needed because the engine might change its time management algorithm when pondering is allowed.
		* <id> = OwnBook, type check
			this means that the engine has its own book which is accessed by the engine itself.
			if this is set, the engine takes care of the opening book and the GUI will never
			execute a move out of its book for the engine. If this is set to false by the GUI,
			the engine should not access its own book.
		* <id> = MultiPV, type spin
			the engine supports multi best line or k-best mode. the default value is 1
		* <id> = UCI_ShowCurrLine, type check, should be false by default,
			the engine can show the current line it is calculating. see "info currline" above.
		* <id> = UCI_ShowRefutations, type check, should be false by default,
			the engine can show a move and its refutation in a line. see "info refutations" above.
		* <id> = UCI_LimitStrength, type check, should be false by default,
			The engine is able to limit its strength to a specific Elo number,
		   This should always be implemented together with "UCI_Elo".
		* <id> = UCI_Elo, type spin
			The engine can limit its strength in Elo within this interval.
			If UCI_LimitStrength is set to false, this value should be ignored.
			If UCI_LimitStrength is set to true, the engine should play with this specific strength.
		   This should always be implemented together with "UCI_LimitStrength".
		* <id> = UCI_AnalyseMode, type check
		   The engine wants to behave differently when analysing or playing a game.
		   For example when playing it can use some kind of learning.
		   This is set to false if the engine is playing a game, otherwise it is true.
		 * <id> = UCI_Opponent, type string
		   With this command the GUI can send the name, title, elo and if the engine is playing a human
		   or computer to the engine.
		   The format of the string has to be [GM|IM|FM|WGM|WIM|none] [<elo>|none] [computer|human] <name>
		   Examples:
		   "setoption name UCI_Opponent value GM 2800 human Gary Kasparov"
		   "setoption name UCI_Opponent value none none computer Shredder"
		 * <id> = UCI_EngineAbout, type string
		   With this command, the engine tells the GUI information about itself, for example a license text,
		   usually it doesn't make sense that the GUI changes this text with the setoption command.
		   Example:
			"option name UCI_EngineAbout type string default Shredder by Stefan Meyer-Kahlen, see www.shredderchess.com"
		* <id> = UCI_ShredderbasesPath, type string
			this is either the path to the folder on the hard disk containing the Shredder endgame databases or
			the path and filename of one Shredder endgame datbase.
	   * <id> = UCI_SetPositionValue, type string
	      the GUI can send

 */
pub fn option() -> () { todo!() }
