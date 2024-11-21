# Rust Canister demo

## demo1

It's better to have two terminal windows/panels side by side.

On the first terminal:

```
dfx start --clean
dfx deploy demo1
...
2024-11-19 23:12:30.479609 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] New random number generated: 87
2024-11-19 23:12:35.716392 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] New random number generated: 239
...
```

on the second terminal, repeat the following command:

```
dfx canister call demo1 get_current_number
(171 : nat)
```

## demo2

```
dfx start --clean --background
dfx deploy demo2
...
2024-11-21 15:28:57.394223 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] Generated new random proposal number: 133_135
2024-11-21 15:29:00.708369 UTC: [Canister bkyz2-fmaaa-aaaaa-qaaaq-cai] Proposal info obtained through HTTPS outcall: {"action":"AddOrRemoveNodeProvider","action_nns_function":null,"deadline_timestamp_seconds":1727708502,"decided_timestamp_seconds":1727682025,"executed_timestamp_seconds":1727682025,"failed_timestamp_seconds":0,"failure_reason":null,"id":113261,"known_neurons_ballots":[{"id":"27","name":"DFINITY Foundation","vote":1},{"id":"28","name":"Internet Computer Association","vote":1},{"id":"4966884161088437903","name":"Synapse.vote (original)","vote":2},{"id":"5967494994762486275","name":"Arthur\u2019s Neuron (used to be cycle_dao)","vote":1},{"id":"14231996777861930328","name":"ICDevs.org","vote":1},{"id":"428687636340283207","name":"CryptoIsGood","vote":1},{"id":"10843833286193887500","name":"Anvil","vote":1},{"id":"55674167450360693","name":"ICPL.app","vote":1},{"id":"7766735497505253681","name":"Inactive (Request Removal) - The Fools' Court","vote":0},{"id":"12860062727199510685","name":"ysyms","vote":1},{"id":"8959053254051540391","name":"The Accumulators\u2019 Neuron","vote":1},{"id":"6362091663310642824","name":"RawTech Venture","vote":1},{"id":"8777656085298269769","name":"Paul Young","vote":1},{"id":"5728549712200490799","name":"ICPMANUAL","vote":1},{"id":"13538714184009896865","name":"8yeargangDAO","vote":1},{"id":"12911334408382674412","name":"John Wiegley","vote":1},{"id":"13765488517578645474","name":"Isaac Valadez","vote":1},{"id":"5944070935127277981","name":"Krzysztof \u017belazko","vote":1},{"id":"12890113924500239096","name":"Always Rejects","vote":0},{"id":"11053086394920719168","name":"Nicolas.ic","vote":1},{"id":"16335946240875077438","name":"Smaug\u2019s Neuron for Retail Investors","vote":1},{"id":"11595773061053702367","name":"ICLight.io","vote":1},{"id":"5553849921138062661","name":"Synapse.vote (NEW)","vote":2},{"id":"16737374299031693047","name":"Taggr Network","vote":2},{"id":"2649066124191664356","name":"CodeGov","vote":1},{"id":"10323780370508631162","name":"Sonic AMM","vote":1},{"id":"6914974521667616512","name":"Rakeoff.io","vote":1},{"id":"11974742799838195634","name":"$STACK","vote":1},{"id":"17682165960669268263","name":"OpenChat","vote":1},{"id":"4714336137769716208","name":"ELNA AI","vote":0},{"id":"1767081890685465163","name":"GEEKFACTORY","vote":1},{"id":"2776371642396604393","name":"ICP Hub M\u00e9xico","vote":0},{"id":"5132308922522452058","name":"ICP Hub Poland","vote":1},{"id":"7902983898778678943","name":"Jerry Banfield","vote":1},{"id":"433047053926084807","name":"WaterNeuron","vote":1},{"id":"1100477100620240869","name":"ICP Hub Bulgaria","vote":1},{"id":"7446549063176501841","name":"Gold DAO","vote":1},{"id":"8571487073262291504","name":"NeuronPool","vote":1},{"id":"16459595263909468577","name":"LORIMER","vote":0},{"id":"16122208542864232355","name":"B3Pay","vote":1},{"id":"3172308420039087400","name":"ZenithCode","vote":0},{"id":"12093733865587997066","name":"Aviate Labs","vote":2}],"latest_tally":{"no":2691403148726542,"timestamp_seconds":1727705361,"total":48076144399574545,"yes":45205991825295569},"payload":{"change":{"ToAdd":"x7uok-pi537-itm37-unjn3-ewkze-kuetg-kptap-nuqak-auq7z-tn5ey-dqe"}},"proposal_id":133135,"proposal_timestamp_seconds":1727361894,"proposer":"8900235041015033481","reject_cost_e8s":1000000000,"reward_status":"SETTLED","settled_at":1727712000,"status":"EXECUTED","summary":"Register a node provider 'Protocol16', in line with the announcement and discussion at [https://forum.dfinity.org/t/new-node-provider-proposals/16643/493?u=protocol16](https://forum.dfinity.org/t/new-node-provider-proposals/16643/493?u=protocol16).\n\nThe self-declaration documentation is available at [https://wiki.internetcomputer.org/wiki/User:Protocol16](https://wiki.internetcomputer.org/wiki/User:Protocol16) with SHA256 hash `d433497932de39d880974dc0e50ae34cf4227ae53d0eaf3f364c83c16eff06e6`.\n\nThe proof of identity is available at [https://wiki.internetcomputer.org/wiki/User:Protocol16](https://wiki.internetcomputer.org/wiki/User:Protocol16) with SHA256 hash `ba5d2bead0ad9dd5777287257607ad79678d8638a76bb6af2c46b3eb855daf8b`.","title":"Add Node Provider: Protocol16","topic":"TOPIC_PARTICIPANT_MANAGEMENT","updated_at":"2024-09-30T16:01:36.029098","url":"https://forum.dfinity.org/t/new-node-provider-proposals/16643/493?u=protocol16"}
...
```
