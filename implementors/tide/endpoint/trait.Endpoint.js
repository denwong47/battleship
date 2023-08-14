(function() {var implementors = {
"battleship":[["impl&lt;State&gt; Endpoint&lt;State&gt; for <a class=\"struct\" href=\"battleship/app/hooks/new_board/struct.NewBoardHook.html\" title=\"struct battleship::app::hooks::new_board::NewBoardHook\">NewBoardHook</a><span class=\"where fmt-newline\">where\n    State: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"],["impl&lt;State&gt; Endpoint&lt;State&gt; for <a class=\"struct\" href=\"battleship/app/hooks/board_status/struct.BoardStatusHook.html\" title=\"struct battleship::app::hooks::board_status::BoardStatusHook\">BoardStatusHook</a><span class=\"where fmt-newline\">where\n    State: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"],["impl&lt;State&gt; Endpoint&lt;State&gt; for <a class=\"struct\" href=\"battleship/app/hooks/app_status/struct.AppStatusHook.html\" title=\"struct battleship::app::hooks::app_status::AppStatusHook\">AppStatusHook</a><span class=\"where fmt-newline\">where\n    State: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"],["impl&lt;State&gt; Endpoint&lt;State&gt; for <a class=\"struct\" href=\"battleship/app/hooks/list_boards/struct.ListBoardsHook.html\" title=\"struct battleship::app::hooks::list_boards::ListBoardsHook\">ListBoardsHook</a><span class=\"where fmt-newline\">where\n    State: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"],["impl&lt;State&gt; Endpoint&lt;State&gt; for <a class=\"struct\" href=\"battleship/app/hooks/strike/struct.StrikeHook.html\" title=\"struct battleship::app::hooks::strike::StrikeHook\">StrikeHook</a><span class=\"where fmt-newline\">where\n    State: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"],["impl&lt;State&gt; Endpoint&lt;State&gt; for <a class=\"struct\" href=\"battleship/app/hooks/drop_board/struct.DropBoardHook.html\" title=\"struct battleship::app::hooks::drop_board::DropBoardHook\">DropBoardHook</a><span class=\"where fmt-newline\">where\n    State: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"],["impl&lt;State&gt; Endpoint&lt;State&gt; for <a class=\"struct\" href=\"battleship/app/hooks/simulated_failure/struct.SimulatedFailureHook.html\" title=\"struct battleship::app::hooks::simulated_failure::SimulatedFailureHook\">SimulatedFailureHook</a><span class=\"where fmt-newline\">where\n    State: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"],["impl&lt;T, State&gt; Endpoint&lt;State&gt; for <a class=\"struct\" href=\"battleship/app/app_hook/struct.AppHook.html\" title=\"struct battleship::app::app_hook::AppHook\">AppHook</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T: Endpoint&lt;State&gt;,\n    State: 'static + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,</span>"],["impl&lt;State&gt; Endpoint&lt;State&gt; for <a class=\"struct\" href=\"battleship/app/hooks/termination/struct.TerminationHook.html\" title=\"struct battleship::app::hooks::termination::TerminationHook\">TerminationHook</a><span class=\"where fmt-newline\">where\n    State: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.71.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> + 'static,</span>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()