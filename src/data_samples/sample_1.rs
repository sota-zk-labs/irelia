use std::str::FromStr;
use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;
use crate::contracts_caller::types::VerifyFriTransactionInput;

pub fn sample1() -> (VerifyFriTransactionInput, String) {
    let proof_data = MoveValue::Vector(
        vec![
            MoveValue::U256(U256::from_str("732760739612308100049906584047157783110714348888046202826270876912749598168").unwrap()),
            MoveValue::U256(U256::from_str("2338447598008876670954408114629385874830756656403809740235239540761808549504").unwrap()),
            MoveValue::U256(U256::from_str("638556572290486187753196286237442199190294557105040487593893779120969081488").unwrap()),
            MoveValue::U256(U256::from_str("1560022457373993664620468419788331868837658596964560113957949278339961003479").unwrap()),
            MoveValue::U256(U256::from_str("488775989980186780338997630358691189773662780715959734843914642824287457150").unwrap()),
            MoveValue::U256(U256::from_str("2226574320078370713032093352652975126739611360964678968588807817522329842825").unwrap()),
            MoveValue::U256(U256::from_str("2306852593192277452696224567240502847832055179648525083552593101645241869971").unwrap()),
            MoveValue::U256(U256::from_str("3472179235766154838434579711205657159041991595865937080682399597754573232441").unwrap()),
            MoveValue::U256(U256::from_str("1078811873943764892026059989281359364229959265376607547640844546763733642580").unwrap()),
            MoveValue::U256(U256::from_str("2321547384977231267188281809340936263451819538224694499302431981292523952839").unwrap()),
            MoveValue::U256(U256::from_str("396506576250321341311648063955301005779204167693615039340334916880455957765").unwrap()),
            MoveValue::U256(U256::from_str("1968679894746215596443429273082638273230425887216288383216633131591329585111").unwrap()),
            MoveValue::U256(U256::from_str("3382230657072057965757469887230919947278019533975846497395189942665749793893").unwrap()),
            MoveValue::U256(U256::from_str("1372726906340320359754829656383702772889616887275002011690574747988728995550").unwrap()),
            MoveValue::U256(U256::from_str("150065324774634707579099029180307454370480205268689392952418091996225196906").unwrap()),
            MoveValue::U256(U256::from_str("3451415773821267557171776638841348908987774750256724545355898848297761714011").unwrap()),
            MoveValue::U256(U256::from_str("267493715973173520137729946743700873520151349798462045235219687248252717929").unwrap()),
            MoveValue::U256(U256::from_str("1350029781887954988918868638236712584840524948775591756228015179532925636209").unwrap()),
            MoveValue::U256(U256::from_str("2033959568180207193617265217126303751378049450303445937070151091346441983816").unwrap()),
            MoveValue::U256(U256::from_str("1759046280763564047482135773004252637790798834143069146917632628771131403888").unwrap()),
            MoveValue::U256(U256::from_str("832860108441282049273441613953043646439890145855897997074121361309246926108").unwrap()),
            MoveValue::U256(U256::from_str("3259061487805176187049200388349468399004395860492090070831623810058498880878").unwrap()),
            MoveValue::U256(U256::from_str("16045754007376442276411162383919808795781809959846918478916552560260947600").unwrap()),
            MoveValue::U256(U256::from_str("1919550401839599907213679961682240847323459033745128193518592511113373324711").unwrap()),
            MoveValue::U256(U256::from_str("344865302256255248290841155532676934758027902395073058929647380768343504456").unwrap()),
            MoveValue::U256(U256::from_str("1308813998568520478930508065159910277780673763321704825002221420562259146235").unwrap()),
            MoveValue::U256(U256::from_str("2637186192668429942905471141369815921708017028565565133033422677087183559146").unwrap()),
            MoveValue::U256(U256::from_str("901600540878637026265055640570027526935243769732114998898326672838432891138").unwrap()),
            MoveValue::U256(U256::from_str("180533624594242831517051553101312105783756133334435279789608587085352741205").unwrap()),
            MoveValue::U256(U256::from_str("1130124878889067962321316838319936072823603507795524179607806893327490202989").unwrap()),
            MoveValue::U256(U256::from_str("227775348505773234034870440594247730467578420661903277029863181902068052530").unwrap()),
            MoveValue::U256(U256::from_str("3258500569336696664293407175339158594662942265854978861477768001827957808537").unwrap()),
            MoveValue::U256(U256::from_str("1016175779329597067412139761120729306653279008166815524377884258471290387629").unwrap()),
            MoveValue::U256(U256::from_str("1672373475343457863474524195289625613193817137321634988969329982055492605881").unwrap()),
            MoveValue::U256(U256::from_str("784396279877789239626135564210194383657057511207828232848610618012009464894").unwrap()),
            MoveValue::U256(U256::from_str("2318204926736945130167472261621413091796612381112572912286347717645630696535").unwrap()),
            MoveValue::U256(U256::from_str("977562448427365711715708718123914784609792131703648629091881075738335239607").unwrap()),
            MoveValue::U256(U256::from_str("249113228450294526795544620887194515544064477514412499360054426425258838563").unwrap()),
            MoveValue::U256(U256::from_str("3408749293742637593905960949969969659542745209625238676320687278856648035669").unwrap()),
            MoveValue::U256(U256::from_str("2275383394714085644159569493909698620020788897726292060869643123848320032040").unwrap()),
            MoveValue::U256(U256::from_str("1410015480582166465653302427109016412489280288873218999042793937181920367159").unwrap()),
            MoveValue::U256(U256::from_str("893033741806628679651763142664970048243838845151361667768906506099875966973").unwrap()),
            MoveValue::U256(U256::from_str("3385150968012464794816192940673314535346902404249108018529099192781100148745").unwrap()),
            MoveValue::U256(U256::from_str("3078865843213230602540993466675248506455008928402530662009658643709402245032").unwrap()),
            MoveValue::U256(U256::from_str("2201348662179545169134389468444779773286239865205167102728358911713365092270").unwrap()),
            MoveValue::U256(U256::from_str("249154481998266292241682790014576345745966483615649632886375425717747720128").unwrap()),
            MoveValue::U256(U256::from_str("1166993418982130549812525063517673144407844553939359706794801639976651182017").unwrap()),
            MoveValue::U256(U256::from_str("2883227819624228151460580420683677916547276517414533744759606658090231351298").unwrap()),
            MoveValue::U256(U256::from_str("3440001993271458528817826783707763532641976780610963062414820706525500691140").unwrap()),
            MoveValue::U256(U256::from_str("2636783663312794536593386201765274285682554263289950872276146085258638105447").unwrap()),
            MoveValue::U256(U256::from_str("2182093569018220342508762184701792118794567874411565930443394123456788727081").unwrap()),
            MoveValue::U256(U256::from_str("2960030828398354867640668653708779714225291533581639001846880096592979935473").unwrap()),
            MoveValue::U256(U256::from_str("2421263317144373664512537164275790715693590494234228177529752160786785885523").unwrap()),
            MoveValue::U256(U256::from_str("1423437469147696526047456924361914663354630737042167642760615032462333208925").unwrap()),
            MoveValue::U256(U256::from_str("258500037246654331886083846164080832282491358291349007339529803383698647779").unwrap()),
            MoveValue::U256(U256::from_str("1390250760701923057645441280460576600508264987349848974642264794686547805924").unwrap()),
            MoveValue::U256(U256::from_str("1133345576039136117336381536567387552825895459972482981799367699327931261669").unwrap()),
            MoveValue::U256(U256::from_str("2277410994385721107546856070952836809538552659559870367814304812158930610038").unwrap()),
            MoveValue::U256(U256::from_str("1060806370357613580307854981257787102967814801511121015599577288051388006830").unwrap()),
            MoveValue::U256(U256::from_str("2152697447236218565223797830941477339296844030779705088388614659428547289619").unwrap()),
            MoveValue::U256(U256::from_str("1296308185668586327126942576294973464592835893173555381549815233827821674122").unwrap()),
            MoveValue::U256(U256::from_str("2414361325547869316551563296637692672188553624183296592991726703734491614940").unwrap()),
            MoveValue::U256(U256::from_str("2476328822436766450159451345607799492762927136891545932609674334001122733615").unwrap()),
            MoveValue::U256(U256::from_str("1985598347737655921604560397846127353854517502114499142903903976320263440407").unwrap()),
            MoveValue::U256(U256::from_str("169835116457147402951142552629357406178039830308352943607711798794596555174").unwrap()),
            MoveValue::U256(U256::from_str("134561089816456934655780106542018741435698016410947688581485306786085881953").unwrap()),
            MoveValue::U256(U256::from_str("1203597762336374060202354349555698492997041604611097740518257265298145224069").unwrap()),
            MoveValue::U256(U256::from_str("3272940371973108962582782466811896182238976124429953703088649042425447566956").unwrap()),
            MoveValue::U256(U256::from_str("3545970982844847052643288226158340897568489162123073787678983387916653880780").unwrap()),
            MoveValue::U256(U256::from_str("1411639025185188744025698943433223440944727246755781436553999362281206822052").unwrap()),
            MoveValue::U256(U256::from_str("3204970823995748722565777136172563653929906606186141290963696057498962846469").unwrap()),
            MoveValue::U256(U256::from_str("1537920829295295799307255765718251979863865838310301406520158686798139305080").unwrap()),
            MoveValue::U256(U256::from_str("730720183998877596624788949943566686225088171654665915766322726264422767888").unwrap()),
            MoveValue::U256(U256::from_str("2598193165152133261698386383178852182755923361168144092349431003738590465975").unwrap()),
            MoveValue::U256(U256::from_str("9225870062283163643868761848836766945060247833351789216705866227624676438").unwrap()),
            MoveValue::U256(U256::from_str("2221214166452930869125775852872969760068612402503120325359312598340174255747").unwrap()),
            MoveValue::U256(U256::from_str("2971843402961368954590318983636866259554823784827412945892358440235470879653").unwrap()),
            MoveValue::U256(U256::from_str("1069086653667961785472672804603128442075547658908073030278661741718927267714").unwrap()),
            MoveValue::U256(U256::from_str("1867320977583477782337041705327996428568434075622690033106591457633416397723").unwrap()),
            MoveValue::U256(U256::from_str("3019224654013907397560903150070715123692969133763466549777583614967133981428").unwrap()),
            MoveValue::U256(U256::from_str("234547459265058059688187102543620462055036578784712714284314184847230539008").unwrap()),
            MoveValue::U256(U256::from_str("1052382013055978253478444205878791945360774497203916540207343990638479964007").unwrap()),
            MoveValue::U256(U256::from_str("2471452992961794771995558620386560106991680527035808285373369900208303317609").unwrap()),
            MoveValue::U256(U256::from_str("3044237569942223895835185054557367797354096935812930636079761900666066650915").unwrap()),
            MoveValue::U256(U256::from_str("2836554476964769863826293055000365537428309988223526229898978561539374931641").unwrap()),
            MoveValue::U256(U256::from_str("3245426627299617332166962151565041129734243347562338196140421602368411547713").unwrap()),
            MoveValue::U256(U256::from_str("2753878971938587878914908258732044778500405546787961710461021412789140572814").unwrap()),
            MoveValue::U256(U256::from_str("2300564061219776034007147984951197516612837858783742913729042739978888031037").unwrap()),
            MoveValue::U256(U256::from_str("169898012579507026449553934826352411905502755087500699476876223884490714019").unwrap()),
            MoveValue::U256(U256::from_str("2585445679329966610281585019317421821949531182167568964790352910424601788342").unwrap()),
            MoveValue::U256(U256::from_str("3314159857314988202500778688080708095074777598221474935898605552093829847864").unwrap()),
            MoveValue::U256(U256::from_str("76890601900160654391173487575924289054531270273482748498955661008085446557696").unwrap()),
            MoveValue::U256(U256::from_str("28951825798470463656369354931594974190285670571006168129961372439822544666624").unwrap()),
            MoveValue::U256(U256::from_str("93145385271435086251900341043870384042517493612831787708862239166905825886208").unwrap()),
            MoveValue::U256(U256::from_str("66923118917532383037660905289029915671010060788500092279817968773240660688896").unwrap()),
            MoveValue::U256(U256::from_str("5601094558871393084086079467005979760322703565816886774229669677245929619456").unwrap()),
            MoveValue::U256(U256::from_str("37994026881238091749584100113688151178427612059922769221572142962916456398848").unwrap()),
            MoveValue::U256(U256::from_str("62909192895601672020618314199482287758204345549762726029122523468902965444608").unwrap()),
            MoveValue::U256(U256::from_str("54643179649329660002824928941090254407873613659310396757669989821107312525312").unwrap()),
            MoveValue::U256(U256::from_str("40894983951613460764524216013972871623091167160791935313899661147302994640896").unwrap()),
            MoveValue::U256(U256::from_str("63300720062659416315034813687034907221501490313541081633579374390471519371264").unwrap()),
            MoveValue::U256(U256::from_str("79915481014682984344027423521317861693682967099698299861958165252326253658112").unwrap()),
            MoveValue::U256(U256::from_str("103700380785899053032231429209566089223467430199922401417401348046139405893632").unwrap()),
            MoveValue::U256(U256::from_str("28859861259175879471828113494011961416649423253652296722050682727877476089856").unwrap()),
            MoveValue::U256(U256::from_str("60529428021482789183736537771593621955340954298377799420995209836354598862848").unwrap()),
            MoveValue::U256(U256::from_str("111010085245744977141061156957134477393165735160472159095078544980654805745664").unwrap()),
            MoveValue::U256(U256::from_str("24172676853046036836329983701189738573423357745841104036021198403824447913984").unwrap()),
            MoveValue::U256(U256::from_str("63401492333866799468976786047668330247365085275292703152968718184401118167040").unwrap()),
            MoveValue::U256(U256::from_str("100515595210560856476979746904673781569218446145158102628315492814354319933440").unwrap()),
            MoveValue::U256(U256::from_str("22723671336562612944086893304714812591123017843344654249464448934751980486656").unwrap()),
            MoveValue::U256(U256::from_str("108550097292081696153359139130692540256021540875455684127124935516815112536064").unwrap()),
            MoveValue::U256(U256::from_str("4357844704525617802732011033826408159229758440882064041139377120917233598464").unwrap()),
            MoveValue::U256(U256::from_str("115317825720353045292190328112615858108465093126915797493968526651932262006784").unwrap()),
            MoveValue::U256(U256::from_str("17177635450669318330966367852162198846462181076275519748195337029267971112960").unwrap()),
            MoveValue::U256(U256::from_str("14300615902126688278160861846695061226488272360317538535029993354639766978560").unwrap()),
            MoveValue::U256(U256::from_str("68210246324623309451647770972842689575089608892511994973926378389041745756160").unwrap()),
            MoveValue::U256(U256::from_str("98006232981289090302941952807095795116342422415893199891453556586683524186112").unwrap()),
            MoveValue::U256(U256::from_str("35505728897003609232602689599448188701697431110221509847618095129777982668800").unwrap()),
            MoveValue::U256(U256::from_str("99247719312493621191216237138715418326047182359114924954277240968831249154048").unwrap()),
            MoveValue::U256(U256::from_str("94773302829938184098267674046972282351717911251742906141277819560064299565056").unwrap()),
            MoveValue::U256(U256::from_str("11533535448849096551073919565303581739172901376199071386901284060590791196672").unwrap()),
            MoveValue::U256(U256::from_str("72013470930071927689571417020584630954726606435014736219667691535686973784064").unwrap()),
            MoveValue::U256(U256::from_str("81366984756620487242087727521570511372019046122567093553809796062903808294912").unwrap()),
            MoveValue::U256(U256::from_str("31553690834848665785135364493180715629840897940645096016867057446683412529152").unwrap()),
            MoveValue::U256(U256::from_str("33029591538002884406278456134268892007453664436380138850783547830911214026752").unwrap()),
            MoveValue::U256(U256::from_str("23007612059980365748768644097143500608953148767878486604441136995276056166400").unwrap()),
            MoveValue::U256(U256::from_str("27403190147157982139224748190408928796603433306013731441174832686315109089280").unwrap()),
            MoveValue::U256(U256::from_str("73353988529160710413867864854303840515375332474148459341482685918530706079744").unwrap()),
            MoveValue::U256(U256::from_str("8863133246912073919725729664844694757854869325273629849180981472946018058240").unwrap()),
            MoveValue::U256(U256::from_str("106731546803291876004613481679366856178792288548104701236748261954284035244032").unwrap()),
            MoveValue::U256(U256::from_str("90669340528385738153876711353407966053393833362943740789059938184310158262272").unwrap()),
            MoveValue::U256(U256::from_str("94351959191024090532276840756405772278913618861656425746351774461298923798528").unwrap()),
            MoveValue::U256(U256::from_str("13002368248557070093345975526193129152931693756507030378972261307968007438336").unwrap()),
            MoveValue::U256(U256::from_str("73885102379370265882899501399139246387277325098860691414646190417229574569984").unwrap()),
            MoveValue::U256(U256::from_str("24542800480304932028352158726021429199102088647155207796387385543530339368960").unwrap()),
            MoveValue::U256(U256::from_str("35098348268927815882498810148773628339334002779352017488506186195608281284608").unwrap()),
            MoveValue::U256(U256::from_str("105072401490029779668187748513559826553379525545751518984629772290513663164416").unwrap()),
            MoveValue::U256(U256::from_str("102094388824294191351768748288791002607796382828294579347228684485096468643840").unwrap()),
            MoveValue::U256(U256::from_str("22575073094160474128193761665775313518628617296966234964982505448265440296960").unwrap()),
            MoveValue::U256(U256::from_str("54719678532908290406650199610833569087816119239684696396527856829022001430528").unwrap()),
            MoveValue::U256(U256::from_str("82466741908136861632941961671838635168054285094684843481316260699376122331136").unwrap()),
            MoveValue::U256(U256::from_str("15189527755055422383089451286540096443718575499544038891427539161496699273216").unwrap()),
            MoveValue::U256(U256::from_str("10387969987781677490783214504823745473919162969162462445081752738718942232576").unwrap()),
            MoveValue::U256(U256::from_str("75537072253761496646449566421689875867780031961228128188038249087926647390208").unwrap()),
            MoveValue::U256(U256::from_str("41967463686585267874106621335234319048417631600921488220321739121240771133440").unwrap()),
            MoveValue::U256(U256::from_str("11184884091912422143201333657384466339594093696771513297082178017696059228160").unwrap()),
            MoveValue::U256(U256::from_str("1537759790339621306152862554670867450932886007341365250781192886756157947904").unwrap()),
            MoveValue::U256(U256::from_str("46886339846340228004196120033506833537209493176283232662896464535949246726144").unwrap()),
            MoveValue::U256(U256::from_str("52784624446692279950991982544156628784451417326487557617816734869354186801152").unwrap()),
            MoveValue::U256(U256::from_str("38938022765412481306463395288423517672206661291924127542791101233441974255616").unwrap()),
            MoveValue::U256(U256::from_str("77254501810828225518401933038541147530713654226348510966795100365991899561984").unwrap()),
            MoveValue::U256(U256::from_str("5728309279570382091489129544846569442147476270092894670667888629124815650816").unwrap()),
            MoveValue::U256(U256::from_str("42149101614411967109478394890585953081200836286153048717816467177927560134656").unwrap()),
            MoveValue::U256(U256::from_str("48004147803371363381802363945970192358899509295460035660422447377848725405696").unwrap()),
            MoveValue::U256(U256::from_str("96218705869522881613213016116275846030524400877096346564799635432483946758144").unwrap()),
            MoveValue::U256(U256::from_str("84192715110664973316759820087924669450057425548357705819724243181119003951104").unwrap()),
            MoveValue::U256(U256::from_str("38873906982163031960302224903303189145095953782162107739281339354939545092096").unwrap()),
            MoveValue::U256(U256::from_str("69111307204538712231544308338387878329704112116296470605062925630630558433280").unwrap()),
            MoveValue::U256(U256::from_str("54682510791781575482551417219351666278017040855716314467949639185980521971712").unwrap()),
            MoveValue::U256(U256::from_str("114574442615783184300274072706242909487906380319472029892314306483489964294144").unwrap()),
            MoveValue::U256(U256::from_str("114813048609331945143458625588376082591249811541207100336937895888544226869248").unwrap()),
            MoveValue::U256(U256::from_str("47912160173146632519631275169088263793009691898871686064432327840955128348672").unwrap()),
            MoveValue::U256(U256::from_str("6639056206167440201329187999255794588153887003571015212713721618327985455104").unwrap()),
            MoveValue::U256(U256::from_str("26457118856390199106825382332801919586112139693201858897098624446857914351616").unwrap()),
            MoveValue::U256(U256::from_str("66293407024099532386841133844282440900015707770593823350087359131572781449216").unwrap()),
            MoveValue::U256(U256::from_str("56719656442774069529025169110686198056364581621226905312116463543032439898112").unwrap()),
            MoveValue::U256(U256::from_str("58501032360052653085393975510933585065061940325117103759602397376714729586688").unwrap()),
            MoveValue::U256(U256::from_str("112539957873022536750283707432633705296690845086265129134144985044502425108480").unwrap()),
            MoveValue::U256(U256::from_str("69113254518834274536103261051596567421852765442803906806584364413420329500672").unwrap()),
            MoveValue::U256(U256::from_str("101482138972401600788429219669766998179346069770849922795153404369508100997120").unwrap()),
            MoveValue::U256(U256::from_str("35522271978317548065624161127503886579075336515170475684298053146293733163008").unwrap()),
            MoveValue::U256(U256::from_str("14922327040860854633826098379424667287340527635987743791090219499426609627136").unwrap()),
            MoveValue::U256(U256::from_str("97280385865253185245724740263198329206320603092247005005822401127277393870848").unwrap()),
            MoveValue::U256(U256::from_str("14018116634043680998372686155590241472400429819508644686840444939877404901376").unwrap()),
            MoveValue::U256(U256::from_str("99493025085141591031945331063157622467245330248327409119010298061025473200128").unwrap()),
            MoveValue::U256(U256::from_str("59341618779705626448420190480279489363574589824825021978526030742120503967744").unwrap()),
            MoveValue::U256(U256::from_str("69618473214628358097999855414985888418313055814079116581341497239400760737792").unwrap()),
            MoveValue::U256(U256::from_str("29142762731837157475933931618855917341009778385349511580817846669306100711424").unwrap()),
            MoveValue::U256(U256::from_str("98155492537457150118311847841645382016565681283298135401014283578507423383552").unwrap()),
            MoveValue::U256(U256::from_str("12561824058702667249435617416993540091039187306700952857794268373114385596416").unwrap()),
            MoveValue::U256(U256::from_str("51565135592278722351325934697075386848462051869780198159629752931586003697664").unwrap()),
            MoveValue::U256(U256::from_str("69979676134069579218571058930276270862701936934223935370733412043783516717056").unwrap()),
            MoveValue::U256(U256::from_str("108657328282277802354558025463339819424363091560174656893534855832410536280064").unwrap()),
            MoveValue::U256(U256::from_str("66438204530674395962054503288603050425741700842441066854881171963458011267072").unwrap()),
            MoveValue::U256(U256::from_str("64131362067177888437207812098106844179865280529305744301973264821547956174848").unwrap()),
            MoveValue::U256(U256::from_str("46577914729266175406677411468375276090071868737620475012100798923180397821952").unwrap()),
            MoveValue::U256(U256::from_str("23405217046433683422781253701987444979054411621263686439347339887254851026944").unwrap()),
            MoveValue::U256(U256::from_str("64544468677253839786675775411039371097795217101958754267139471054074185515008").unwrap()),
            MoveValue::U256(U256::from_str("113931171266937219620609785879270964604992456272345120306688393130923274534912").unwrap()),
            MoveValue::U256(U256::from_str("111143557704057102191391414138246547725307706309972543569380721168830934548480").unwrap()),
            MoveValue::U256(U256::from_str("110860655711444349854461643816225722843923446948770261067257023595879164018688").unwrap()),
            MoveValue::U256(U256::from_str("46840717312198268492084117480132321914024177609728532251645337403454142808064").unwrap()),
            MoveValue::U256(U256::from_str("68440390247805339410175106749096156575753570593239158880662732987535384903680").unwrap()),
            MoveValue::U256(U256::from_str("66501134395927002834264032907260669568642171172362409493089229481845494644736").unwrap()),
            MoveValue::U256(U256::from_str("78610869832528399727189072777630654688557907079722577505880569386728526184448").unwrap()),
            MoveValue::U256(U256::from_str("114850986350390194987512784706435271781065479323470753998205274719731086000128").unwrap()),
            MoveValue::U256(U256::from_str("52632754682768734903846740408724542847820505663926526929525610273823329878016").unwrap()),
            MoveValue::U256(U256::from_str("24363610071119668937916408936011333833177826593172464535404137128778010722304").unwrap()),
            MoveValue::U256(U256::from_str("68209196664397503312193573711246814134746843615088486481905808012132147527680").unwrap()),
            MoveValue::U256(U256::from_str("99914800062639277712631289053936604978404341905810277090752383926578997886976").unwrap()),
            MoveValue::U256(U256::from_str("15454604719652172672855510878030281439712630441624602285040387558326658924544").unwrap()),
            MoveValue::U256(U256::from_str("81883511606353927931629085486578520968731663621562623521105390579287454646272").unwrap()),
        ]
    );
    let fri_queue = MoveValue::Vector(vec![
        MoveValue::U256(U256::from_str("68842").unwrap()),
        MoveValue::U256(U256::from_str("2964178946110064358991726962687892125811805618747981051340918888528792149605").unwrap()),
        MoveValue::U256(U256::from_str("2179683427920472696291350577048132027566530074554770687235446808381170104740").unwrap()),
        MoveValue::U256(U256::from_str("75704").unwrap()),
        MoveValue::U256(U256::from_str("2103094102915868483382485829173571396270087725632841249210530049472424431841").unwrap()),
        MoveValue::U256(U256::from_str("2479596287378622959513567150060924229204175125306857273715736723758389236318").unwrap()),
        MoveValue::U256(U256::from_str("80587").unwrap()),
        MoveValue::U256(U256::from_str("1798636197358091986020677304753686596407232309857363392050569119004515665288").unwrap()),
        MoveValue::U256(U256::from_str("1276280021350835536877635448007346376375629566169883884721703797525132952779").unwrap()),
        MoveValue::U256(U256::from_str("86673").unwrap()),
        MoveValue::U256(U256::from_str("1223919860614697179665648943701441038121903949120965235434568549706881265714").unwrap()),
        MoveValue::U256(U256::from_str("315285769684436180317884585551657710036431429370134602541400631969325311314").unwrap()),
        MoveValue::U256(U256::from_str("98467").unwrap()),
        MoveValue::U256(U256::from_str("1498837460801827753244815055028767727101955870935055090297998458035615512878").unwrap()),
        MoveValue::U256(U256::from_str("886174992991936454854555347243570745158880282692944820968869513404805391082").unwrap()),
        MoveValue::U256(U256::from_str("101247").unwrap()),
        MoveValue::U256(U256::from_str("401382385200356601733921431908710955160228380430776219606687202903303544395").unwrap()),
        MoveValue::U256(U256::from_str("679814886334400165039259936221294035995228207839446638120957569565762850347").unwrap()),
        MoveValue::U256(U256::from_str("102074").unwrap()),
        MoveValue::U256(U256::from_str("107169631283831902545404617598779270567619544128856151074130368670375305073").unwrap()),
        MoveValue::U256(U256::from_str("1658489653615862222132196732940191116616212573463363599252528863907631515768").unwrap()),
        MoveValue::U256(U256::from_str("112906").unwrap()),
        MoveValue::U256(U256::from_str("524467796028629916135088306351772443014515171851601370770980672297816004568").unwrap()),
        MoveValue::U256(U256::from_str("3054550940202130034617898482251712777924047495173323457299213297263826350828").unwrap()),
        MoveValue::U256(U256::from_str("113760").unwrap()),
        MoveValue::U256(U256::from_str("3376618749243009688521553232710707914835694730136690370108761279729079575438").unwrap()),
        MoveValue::U256(U256::from_str("1058561548859322008983466353839301649035915957137925653166306645978868347203").unwrap()),
        MoveValue::U256(U256::from_str("114976").unwrap()),
        MoveValue::U256(U256::from_str("1227923646611606553177690648991829647219422103852733855662008292391993749234").unwrap()),
        MoveValue::U256(U256::from_str("1899443339253771681117963117804100042655863677622846446674184778656382944186").unwrap()),
        MoveValue::U256(U256::from_str("115525").unwrap()),
        MoveValue::U256(U256::from_str("235160462280836882059278614936884803584031242303237407732165794589913142069").unwrap()),
        MoveValue::U256(U256::from_str("2309872502305684530821077676648359152149200585228250214134985515759438262321").unwrap()),
        MoveValue::U256(U256::from_str("119020").unwrap()),
        MoveValue::U256(U256::from_str("573842828530156600986657311356120176821431616140699737178615667697957614495").unwrap()),
        MoveValue::U256(U256::from_str("743000622407137865621501727376975648288989174439365349123213734935762649486").unwrap()),
        MoveValue::U256(U256::from_str("124339").unwrap()),
        MoveValue::U256(U256::from_str("1546530425735562875072380887378689929310882182235024380357204143554321321082").unwrap()),
        MoveValue::U256(U256::from_str("2752221987707151453287287390473883776897193346908145369312214213686196014690").unwrap()),
        MoveValue::U256(U256::from_str("0").unwrap()),
    ]);

    (VerifyFriTransactionInput {
        proof: proof_data,
        fri_queue,
        evaluation_point: MoveValue::U256(U256::from_str("1127319757609087129328200675198280716580310204088624481346247862057464086751").unwrap()),
        fri_step_size: MoveValue::U256(U256::from_str("3").unwrap()),
        expected_root: MoveValue::U256(U256::from_str("9390404794146759926609078012164974184924937654759657766410025620812402262016").unwrap()),
    }, "9390404794146759926609078012164974184924937654759657766410025620812402262016".to_string())
}