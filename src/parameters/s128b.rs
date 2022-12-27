use super::Parameters;
use ff::*;


#[derive(PrimeField)]
#[PrimeFieldModulus = "14474011154664525231415395255581126252639794253786371766033694892385558855681"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
pub struct F253([u64; 4]);


pub const PARAMS: Parameters = Parameters {
    power: 3,
    rate: 2,
    capacity: 1,
    output_size: 1,
    n_partial_rounds: 83,
    n_full_rounds: 8,
};

pub const MDS: [[&str; 3]; 3] = [
    ["5862784392132811822399346461209542559968707338020872164521991700364807694530",
     "7757408330399788175303535480446337571071489087910607948806353871463965516666",
     "8496075446250228238445047861615059028340121959933883608591785836214266608488"],
    ["2468935952984307777828540490196237105997853101779803225621880934695688047791",
     "6447190746270133990447879546053525486084787239389374431230881999686004560382",
     "2496707428070521946661419226048740483237118601491693355266478441023446188277"],
    ["2179966417204926823685133856778517752450232708529571105717441010684845408326",
     "10507922981893311351726119744145949418563793888901601023199311329280648882667",
     "8943750249843703572493160575072148049724667463496272400706764847123663685181"],
];
pub const RK: [[&str; 3]; 91] = [
    ["10187801339791605336251748402605479409606566396373491958667041943798551150211",
     "8824452141556477327634835943439996420519135454314677708228322513850226510025",
     "5264468709835621148349527988912247104353814123939106227116596276180070073104"],
    ["13637881690548271929339839559387105015099546141973354320992368464013027112156",
     "7027675418691353855077049716619550622043312043660992344940177187528247727783",
     "13234561482495297019574434398981523055213551324895011246269352068387930806036"],
    ["9807824974813203154912550227282760432817675540785796963800377801121529764316",
     "7164723634800010345170559916055390119875185097834930139818703833970169645199",
     "2720682389492887826569968874051882181427921175723234895884093328329406094669"],
    ["12755161837895618168300590476484610840922267528627671859325836882165381806691",
     "13224952063922250960936823741448973692264041750100990569445192064567307041002",
     "5972677482355664808251970179403621537587317604691542457095312708177712905935"],
    ["13259592869071520474598739447076270973703662109491846304874408686220376925615",
     "11805829143136122768520763814474803657178176758865384785225228950060491370520",
     "12976649114738535452015180480194409647575485052878282497901508667002411696044"],
    ["5990567833466552641396252479972748483750532976297674315563264575711713408919",
     "9855503016642955244574878257541496154292112711726806671346353659135016209587",
     "3518297267402063482291939394705434866313303765090336458957115088213895354374"],
    ["3959032541349471342136457591475742508377376565034118585023229836334458386499",
     "1954876811294862618527744476059844724399371231236154603296457537684819645267",
     "4432315946020764014086040302214874341045344921127163160288423920774120084471"],
    ["3690404384442200903782264995004334807363288367118117498605340707008369192998",
     "13645269838164507597212915259972226260090822169419168596362924385886927079836",
     "13484299981373196201166722380389594773562113262309564134825386266765751213853"],
    ["3617032588734556998784828290112344996993359755978384125831280879049491523420",
     "13479520726208359005162548206217016449657296677090022213672680654064388575909",
     "149101987103211771991327927827692640556911620408176100290586418839323044234"],
    ["11895436329827531142875612937319609768428626807622623298418286599429610076189",
     "2213946951050723319284118699043488343304543214612796775658874763178687874398",
     "6411227692207945049588142150610514607178042116321124498744314021784013167807"],
    ["6830534418395000550371398514530224760386088729335290717464666818519494431137",
     "10580519657430965986108162471030486012424647365859891533956693650987126466818",
     "4232557491102671303375969264426262870173722026524819099819202041658212759409"],
    ["11003656447024841860871485341212795824030062119024725564386935271706539897585",
     "12013791898101584042954224374878193133248473257022502407565361530611313814829",
     "1032400527342368753098340642114835105044001363535341010207460764251901272214"],
    ["11662409070620397985897630435335326247221838512006448323973260294174043580151",
     "462498083559900518334470833812433371588070463145081337823098818653543761968",
     "4622082908476410083286670201138165773322781640914243047922441301693321472984"],
    ["619074932220099567584720641194915509696200347027793052042294963879966262765",
     "2635090520059500019661864086615522409798872905401305311748231832709078452746",
     "4596755424917813089826497731034412067781857175332385741140491598699058381905"],
    ["12622420533971517050761060317049369208980632120901481436392835424625664738526",
     "12518773493257903476892220377586304475073047132217811687051850178035774765525",
     "7150910409873088915567138378833511140267031768668277297762665604766946225946"],
    ["2507023127157092715378409728900319447911047210331698014507132664783822654175",
     "6939655696524592186253894258279685988217778268760320782855939125945184733803",
     "10276258614138325071655762251931930877484678837902919206385184121556946750324"],
    ["7340485916200743279276570085958556798507770452421357119145466906520506506342",
     "6012552570023217029001553744799496723352084228510020003935464928215155080191",
     "13516039926343867345505349297700416735284302629179495705277891580733185024723"],
    ["7221669722700687417346373353960536661883467014204005276831020252277657076044",
     "1553799760191947885401667523639475410796069334686100176383193333680131543539",
     "9981290294385917856729327405448541571580888494635978814465286703665358363584"],
    ["3939248553588280358043372331356972865168261664512705571501488624762566021823",
     "979382242100681784963649758520863053533668631644160532567961063295973792545",
     "5478929644476681096437469958231489102974161353940993351588559414552523375472"],
    ["7573178658386629403632930183877571376596517783760503857587667734606933354565",
     "12487978692778091740985983585442456308838452715926267067150762392592481916055",
     "11777466616075874658540824660073245663131453917311848438659004818029258226188"],
    ["3034358982193367965665811038627621289990627562432985174428827650004786426498",
     "10881134648147910728333436580006586961539390354622077454384678939652780166293",
     "3002662261401574812585941059084045442105770544219592294897623853105950306168"],
    ["6558187162119512771137219685402504304049569060201201820551473403106771487306",
     "4694987398564126756749445490966446527761170564284126078743679070143732109318",
     "9812100862165422922235757591915383485338044715409891361026651619010947646011"],
    ["10802080841949853834350207154609663910756689868701213997346981995894868889056",
     "13052524868531695956982628894833001322405070332024054737192806786755696404408",
     "331415322883529248089844923742652875393424730212767610334384445296044774377"],
    ["5553514022377420464238358833288048268497898485931930247370798710915012134806",
     "14351409833166478418706180703540879023108609875313349666441156566827698057466",
     "8191200994449393414091583810433516255449683673446622353698525941163463918795"],
    ["4890343266376121472310340792124959893872494279005485968116728765698174529959",
     "6183813550940466240069125780194307995419238432953580078105001909257203276029",
     "14220530654250029525801935118720825348081122139509940741635298772154340592685"],
    ["5279094810295095237156880589938713387655737808804323423999071039244700101504",
     "4588607160952144132098306988967507597469812225168616305584872049750719212631",
     "13472532940316345135184512331951905470210820136203866592192820346341185390128"],
    ["8236905392675869043587797310050380994360813912790103626000634297770287551953",
     "13084044495411804426081493256493193251702812210094831941296663580569190057582",
     "9938959468530263238689634115566149670581021441185248861530895506918078896675"],
    ["11692586721586551523329150170263871124807354999073053791819085788475794564268",
     "9358755179172726311342883196642626660155973385865865702703072387015711757634",
     "4390315048845376759299934875153957478778619309709587257015580699964208306451"],
    ["6935839211798937659784055008131602708847374430164859822530563797964932598700",
     "7718575645329982002350668393881722835219810598954183825243771613533940160466",
     "242195042559342457701875247467475541476692066328222777815539458253844365164"],
    ["13710970544888392482670345707698469689492766901394672488284507327884683667372",
     "7144023895814504370698586937504728260736588610927136323237110389085553036898",
     "8557845108834652326975981762589128272026624364693661001203327896381168930462"],
    ["2514191518588385618668928614438843316582435721381741250684215233501480107391",
     "13653844116957219001001661926327517031102957009622534177141534288495806576803",
     "6426970556633164973974750225577406832754186830506033652467796257140393718091"],
    ["3551834385992705076395643110692131686257189926849324538395157161676019274476",
     "1219439673853113792340300173186247996249367102884530407862469123523013083971",
     "11617028270536375525263487484323797546474290465326614733132539502906133336301"],
    ["3355402549169350193996102372535400061798109779657451610751899652786954306325",
     "7436546675341993215489754835783640726765038435314671201608501060242195220586",
     "11047608856641469169830474748676154481406056538978530484831601725107528724215"],
    ["12042527723601346590657884194893851420490506719701838218722677439006972338267",
     "7426530926974171153400957828593153869151416066753580647627979426962467690363",
     "2831653363992089802376157134755132365655942185746372114158718620635070241637"],
    ["12908368178630610710569719268370160806022770178121262243688122486389919287356",
     "5289258967375789615351955821350540105879877430939861109685685406944180736525",
     "14394265608012127462571567392670117594122990580941141122530627662930560237054"],
    ["710549042741320328529708787936108376577483681785348705740417476707343713785",
     "2345013122330543792101611694850613361685745340402695236354732272023036500152",
     "10765375249303706038197004973419243095632341831075144860143874556488156084383"],
    ["4785623617704249555612091116526556055440278365292840697489968061897354304946",
     "10310248934461408645299306991890381824874412097097116172772756260521943896089",
     "7615520301750367991880601783964490200518984744907285331066004719383580213174"],
    ["11188899504408799217967354825394699054734255111797148746588781060130237141725",
     "8964575752972179072552638695299372665438832411303136970325653149436229425353",
     "8205317767954070547119758263562008446573857859342661271430751481649935512865"],
    ["3979162104965576489674205101543710946852828408375514816306435515454435517558",
     "1386580151907702662587737080832885809078061248190237455048089136290189512992",
     "7223391117603518639598440466751080193286510459677016042966533640059408680663"],
    ["9733191209595633506416643105837735959809004192605121386222818936440930309723",
     "486538168871044251085009120658351797400494522648523958245524304862045256691",
     "1558415498960552213241704009433360128041672577274390114589014204605400783336"],
    ["10749063905018409753971835881025233122402531635641702583180695671822925094888",
     "2235429387083112376131077598084080165902381064563107852371526892545960851163",
     "4909399565810006770507426734458619713110201195951888993667213536745190404064"],
    ["8380315914250617110582996255563536054771437886456394663929308867370882705498",
     "3235435208525080082931898983799117422671035465813798340249129617032839553333",
     "1293239921425673430660897025143433077974838969258268884994339615096356996604"],
    ["3854755057793743998382749043935645076321648392889271506938052408708736403452",
     "5352410381163396697819941768893885243535694694324050017399948445792942499242",
     "7539009887104016209392520764993500411036167303958290404628573099048438262045"],
    ["3018926838139220625506489015981329792764782845838536860011466262176793949674",
     "39701437664871189523302235943983519533638233765990726458549906036929754998",
     "908381723021746593339570185608590229480942094680189369741300881253575736021"],
    ["3837565647574361068761459036361038720977149057378981325662868867989693614839",
     "8520135441108714133581970267103801263368853951819485470074941064511220447680",
     "418512623547416088391723876387535911288411841095749808095919869540617448984"],
    ["5410684378713263186548337983252960352059120561867029137014627845977008289113",
     "7607552010319772281480482056612765407544042522430448794355897093565701263964",
     "10560443382562847032903810186773383790191172686921488835536541463612896702880"],
    ["11765584987740430767767720963618789471599706414952870467586329869086306061021",
     "1762188042455633427137702520675816545396284185254002959309669405982213803405",
     "9423367831245402395870732521319058488718383679437757865233865664334989167725"],
    ["13086156091926073523380666471499510015278998772411048213539821848413088237741",
     "11924752273555035815636533686531247827454765648368278679921751482516049180726",
     "13550749835147637257149132251444227574025357160870832060440277657499614035279"],
    ["4980022470210542427146263474888300588899900028640632977248755689173201481217",
     "12194624998206270864758583600243377897592014816955399868337294924074362051319",
     "11646101890917244289289052796357017247643510930320947559911433916880381982804"],
    ["6341903156998053585406994485489167044975338988123396132600439007691254515333",
     "7075683053762049425384470507173220516666842086565372831191898736225176833969",
     "12339183742952719597367108525158230576242909340192236085203045953436828588126"],
    ["6544806624931636820046250739287622854861165137157363228477412109592073724449",
     "4162471908441602722111530391301479454101487411256108519749268293323246853465",
     "3855904019349977168075627829106833431554741216264895289124424891683702161142"],
    ["8081573990172653638630613451663558255120309113082158296697543180884327681628",
     "3770424843537837666617449317328375453122197222151870507160190432180205488340",
     "8536337612042884865823862990558395500695879186323517844891061024641778919517"],
    ["4105549711316030661060813474557643059544637207881308235218224851109489369717",
     "11960391615090990567079593095976721248830913673918120417192162585315739995752",
     "5076891494202411776760085639089021661480026202575079200283033859925084439937"],
    ["11705756475621142531993264933474842610756470013576597133965107051639837169224",
     "13741351539856929954498559924093474280259681098675282580187195151741900723408",
     "3027068551635371872953244299065560957541615113815101131189242325515764690059"],
    ["6850395512313577247119081673505024908539779699924358158552830822936565896447",
     "8956086407974197848637272350322833148022602624785508354504602767996995213788",
     "4790756545207195080071720581233345623221139929363317122620010244898674217389"],
    ["12474740635631417608441859464962458799096036866556081881104510975404284630237",
     "3495680684841851669468757117759016721413638849971831476780828860386585919539",
     "12431669881911492027539297308953445608587717189954390030034316352358291519173"],
    ["13473834488972645181364798464677331389370696501289626421839474973708165241187",
     "9933250710090698648174917102640713736046106810967755531614818701588711061097",
     "930116505665108940369083060127664353583958633476308834849463062893603682173"],
    ["10023394751834506039535835528272848629669745400150361401320009377839386028739",
     "12410769212424127078176119620628096413678537209892760449888079715681785797774",
     "2306241912153324117514359373155364671004781238206203396999313236825587184646"],
    ["411529621724849932999694270803131456243889635467661223241617477462914950626",
     "5160998273928415655409823184165899261172990046635763813454147746797659213144",
     "1853045663799038840844200357682503563381388364149441145393842890943063622009"],
    ["10200060836829532894345177157281490350920059757696303970795575868782116430734",
     "13159315236348307522247320338327508202880603217101108650889411139889606648708",
     "2622104986201989114405870237409757584394794894639900992163117703774646183847"],
    ["2369987021925264551825102643978956756043624271037372491079262595486612463322",
     "11140578593710414034189608336908498732337134094057879227642847613096007368196",
     "13889595442177754598892537082881169375497819074782697987852973860359070672134"],
    ["7706101431683962170748864558736885249954217740028914732545058932435994148261",
     "11215510342262777937959939344707490346990461846904065605590679911793128840184",
     "14294217693107412351976801487758792515428461120505474004428341094115350941322"],
    ["3682076693466701182062369703177588775177371633140590131412886124703375592908",
     "10707887432375252298829520257221060883356326036211033062627152466974818597932",
     "11580067677506341502050232902287564308838930592614842856355490762938012110636"],
    ["880409284677516737794143810282269435436869995982921576549643697413399016444",
     "3625379044328604445017671283561246502850747224428989276925240574779942190412",
     "6320419233799899729388582856796221910956884405787773104057405729484356532577"],
    ["4043654907824840162873903984931870111203178649649374787881862315748593586768",
     "8712558575591178627646966119091406463745130465063969822358568006142648159678",
     "13269280814120791689018246203293707484207916170171977484087989933376123238705"],
    ["4010663644488387734216662043340398912659727136190100684683631872086884884645",
     "4693539785169921996706165279011548411035394830718653170630243145378817566954",
     "13031947796327095660032190165297431412498254986709428126509365542186537851579"],
    ["12269374342586015465639366064459209823001331182668431980583649610423241536108",
     "3623382072093621360324914837010787702508074046403474695113195496868820798538",
     "7011086086203502624189793473666763685083840151819662912121223918053643149084"],
    ["3600854486849487646325182927019642276644093512133907046667282144129939150983",
     "10016930241493632301662250420286427076491301648089728077714543388754509196254",
     "9959705538235431370181375503186495072171574307837686840111188119237864917974"],
    ["8056115392381488214428218511662956255680546327579309477742441707294811958147",
     "10283126821075028181460781612903066863291766350114310674479581227447410732243",
     "4957522379469185879702699424410843035763313862013657992989940540715156607731"],
    ["1933279639657503578406587541357405767411466154336088493966526042743598550902",
     "2138221547112520744699126051903811860205771600821672121643894708182292213541",
     "6313216304209770501412298144859507290401436592549950229914651662192978201247"],
    ["6090236991596262327408212909912734539091116579772880897737743653575530314797",
     "11360412127110486398703016067500520173072536018325425874496685808315346054976",
     "1943979703748280603904302007539344051753490568587165142053590069794471693776"],
    ["684525210957570259428528496229380295065177651980233161490279976442309650151",
     "5324088189464912481605464120209592098785379451464127333628354276113620513718",
     "11047937883714415909395931045871994706853023180590852570362209260425388138594"],
    ["8823499279575388961439813238720128083545335887121293188311250199635211112699",
     "4304575461989675868955670189922380041751133011838287171575479163119733371316",
     "3464340397998075738891129996710075228740496767934137465519455338004332839215"],
    ["10042254753949331598794714340591995019262491069888076724903102165423762006840",
     "11523254830249362162256695767976697970209055038019950577575113071151802550969",
     "4750030501571238574491864210100555679943581925567756526572147831645633743960"],
    ["5527472274416140542876086516990990026889498758922924182449582352244404392867",
     "11880765442983944777971810121968391924028673917382470997783686313133182301624",
     "13008683657916762434211784957458271334210610135613196749049319280362858603121"],
    ["14265766115735107087700873862467899732048437698642616056856966810738489164556",
     "7061471895553716731806082882582404033880670163489323371664740269303775634134",
     "10266959477567992691146166778363477650605633676210357810906146168244851416171"],
    ["85453456084779255583418488188332773800106455407946671095565972122774025681",
     "13586138032575650708060089181624820411154599967512935508933076151074150939058",
     "7415248027993333511520563052816831805939690496204187783003638700824726112592"],
    ["13700097588513101583540301466051375374790979680481208874455054275721973191486",
     "14403384635217532475644232997372226567362383258822841612696232430924751371845",
     "13380513772761377852799171494070693225201198151001629317094611002147573888293"],
    ["6714899802221342233773129092725128944867322380063456869835704189892456118069",
     "4192926855785330265753003361744539845529523638058765674746715426508583520700",
     "5332533484721198492046480129162789413486614526040752390137638282586451567579"],
    ["13195403638201087586578991508241490917987974388312014213355438009537607126430",
     "7181004107638025245090674254447237694978778080067359162508771528174311384279",
     "10924577724685591604709130096892203753353289638684278359027200547953287255192"],
    ["6233005527035139687546287534625755859339280309084303675451600654904917298285",
     "11510755715761414256334967213097949648733086224434822603743581608153979218710",
     "9542241249859856258519642815684180780054580641338599237310424478170080842855"],
    ["8163421647630015580899179925228123364763687136253007407138938462861499930351",
     "11722032101634233887635485313522723603968981327474018550710347763840833882903",
     "5902836857132812261990490241931380400366746014787185691388456551559999954417"],
    ["1812588309302474655043003948507433822433877959260873852114924630684888512493",
     "6455402597285144442503269136658832983246145947877331738383601829541718735231",
     "2989087789022863982526929708593697535256551246269995561487566048169476049252"],
    ["7305061861736451152905029706768892625091314786135577103192983811561528671691",
     "505425339250886013076703361573958920728066749905657099019241672294475109370",
     "12974471912854936826923156822577958087199529911183841151828105075678317871873"],
    ["5872375384986099556601119032006413036325371640197059253395573291196865622648",
     "1625090409149940966858454986530771387891625531896119715417165751264077286965",
     "179139838844450210592009918163124641421696118987114592666298100451353799953"],
    ["5185569806813866855768453225999163395653539737589408493513382157527082430822",
     "2737301854006864489062598733337068689102680351550005405227304969786257236071",
     "13858246582459296231477239162079841391631572818321891290563442036091880378914"],
    ["374029488099466837453096950537275565120689146401077127482884887409712315162",
     "4591906045183610408372262877631309811086136042962623471417232217787699037059",
     "9928973034370434181323659075000155365618775060836009039518331244169306146072"],
    ["10752912372242641369003551421233246649173901396582967842424958999195071410529",
     "148057579455448384062325089530558091463206199724854022070244924642222283388",
     "1541588700238272333689786749850391910886043454136337982301853802587780729085"],
    ["3765516654545142020364147757569169786158936863983138005493898891678512494507",
     "6248781177970865726065396241301623485300380191920728981443436356834033312824",
     "12214557036758035356143663727638374031583047580017985361109749409359257607793"],
    ["2438984569205811206440916967282943047351690380913568868762849415443630295406",
     "9374797833173787558417859982491200734353352114511517126678170643864702861343",
     "2262318076430738829137218755165945363642315922559670529358901778390743783970"],
    ["6411206507247214244488519798493505274870442254394271615004329570020415217037",
     "13149682081791685323229616620075094818031375602196053456362175086227965286321",
     "13625519590726389313330826648331135196773683203806269224524730504117898644586"],
];
