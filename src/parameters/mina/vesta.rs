use crate::parameters::Parameters;
use ff::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "28948022309329048855892746252171976963363056481941647379679742748393362948097"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
pub struct GF([u64; 4]);

pub const PARAMS: Parameters = Parameters {
    power: 7,
    rate: 2,
    capacity: 1,
    output_size: 2,
    n_partial_rounds: 0,
    n_full_rounds: 55,
    mds_matrix: &[
        // row 1
        "28115781186772277486790024060542467295096710153315236019619365740021995624782",
        "22098002279041163367053200604969603243328318626084412751290336872362628294144",
        "10518156075882958317589806716220047551309200159506906232124952575033472931386",
        // row 2
        "8515206633865386306014865142947895502833797732365705727001733785057042819852",
        "19310731234716792175834594131802557577955166208124819468043130037927500684373",
        "361439796332338311597104753147071943681730695313819021679602959964518909239",
        // row 3
        "2193808570710678216879007026210418088296432071066284289131688133644970611483",
        "1201496953174589855481629688627002262719699487577300614284420648015658009380",
        "11619800255560837597192574795389782851917036920101027584480912719351481334717",
    ],
    round_constants: &[
        "2517640872121921965298496967863234221143680281046699148760560696057284005606",
        "3391756047431116221709518926936538303706203177575259437741546230828058541679",
        "28193080211857729746868575888309975056941007202713113547154010421664334143056",
        // next round
        "25261619184426186938919514618416881383323154981235406731208902193655587998749",
        "5438499261516835502981531641588657477212528137520578797088407969732830437134",
        "1447697894671779324954748568939217281372628544919576009518449387265606369859",
        // next round
        "5035532530235542599906399941203951970682478985022204457211063504597080640029",
        "18548939393800290417015907795270784249198528773378593112394621615021029911007",
        "28314657632459005492203969796973258399484591559931227050853551342156833947891",
        // next round
        "10075465805557971120845970058070916255338843492716768289922460436606689369477",
        "21985996556868691161386211003270106475915714625334030557267947035839814254081",
        "9778523497398309788873186849997676949503189428912377745814036481347657299161",
        // next round
        "6085447467925843146276340167082679235758707259098174769103982431882228334038",
        "11214803418623679719680560978819619149235769633101428825693192995405955507848",
        "20585482519401972421539035665320299097144487427998598740316244173221216198246",
        // next round
        "18602266896623204184748247002001496873223612100325866696399863661914256384486",
        "22165919841309962137671309308234475433816142848229812860682345190836583925843",
        "22833505632200982123686653495190412951871851216487329681987951602744930627412",
        // next round
        "200996541962081036547810490655955282117589336000744078845964972887355639644",
        "17159390488590225463405148524511348095493761844950655304775985535830170165304",
        "7519689807382250126180254188667761476713509751388558140260305473388567529705",
        // next round
        "14159331841037307097148990917607709903712709092721125605507719995418592745663",
        "10490695046555645615062072066940833278139280813429718770298136076375411280286",
        "9996921069626538041923613626115903019578182147993504053879837245826104687293",
        // next round
        "28009241574980093348462093077828465154604666812509186537490618830383877236685",
        "18925279443828804264179873719494108834579217607847079902207023181925588871175",
        "13126164514615718686767880517156253918404905174962666942976286681458411835722",
        // next round
        "1125667389564136291825905670957082668987611691949011617627091942772124917554",
        "12737072162917928935765906421286553437026542524142430058538254259863452556200",
        "9855113244149548216327019561589719324434080884827484555441182992249251832158",
        // next round
        "6006604346195593001833550983798183088851044846011297061071167569148810544010",
        "23783465709464699444911580329342599880163107932561352210466223087637763994288",
        "1581060363083815351710754851350813999229829634252940169154424073664057276774",
        // next round
        "24121961545310887440574053281799796355427122479626872394472157625455666323022",
        "23925781309638869606256007860000699567158045595326122474217734988331349678475",
        "433512980570318160778040929743715681206456334448542248765142091911433454703",
        // next round
        "8080307140515367021419180108267113624095868360927897204642243727009503935719",
        "13661807750191096117929173962837770733539092996971801228126331071941306856508",
        "9268394414065063505331314418649987795374055416089324253185088859000252370756",
        // next round
        "22374115023493407761095751712373350824513305398485824175669182288521610150311",
        "22951274634403942446739133926874770994604864227598567536319143390467218980824",
        "21411532836345163980832919797897483979345524322135010935120723250070247464549",
        // next round
        "20688285497159372157224857370703211924056803904697620218749985029000049442943",
        "8350087190167057556241775495760369408781696125331535735138679647687106863977",
        "13485893160159637778707269611856683957779710980787754997470728774769162419576",
        // next round
        "4621792784192688819920303666439776744566536330750316034321950771579978771021",
        "13900656491552343190424687336475573267660717627286734246676255663734655019912",
        "16577037405341365304416318048187907895286388691199320947077947552959834207823",
        // next round
        "17453637937712580666297652202332273322112052411250919589546137386514183913993",
        "9852736110707561006399582579453396957225552488023642073454517393228764176471",
        "8053970357622019747109700798952789019805031210730923951116580579194625334710",
        // next round
        "14566849926060034944494603512439278530775668595134329897253012222562109882008",
        "8863944349051942080060073891691580009950648437676309749771884964336231381737",
        "16455762285584757654310476505019438984453107876908065440396394186006196612077",
        // next round
        "28098375311516838082882166381119795701982164671360574802728073046992978741339",
        "13538346067341652694825445642847479918140731375902310280683284825070643960891",
        "18313412784975078534612748781201087502203257054025866271209086293337241477805",
        // next round
        "24807061345703288899043018750567607387907450632666147403804744880717736838940",
        "16638378638176552952794487891875614248110181610295183306789394461536640085108",
        "2342874860138849081032934096750004917991517717553229739958552529472431319656",
        // next round
        "21631810094765090996871180483650934431972930909326270651252393395613356531282",
        "2220759912186713489010197903069023809260408491503960321105305330086947471014",
        "14815764944505758746761442212662459585220143243155504464852948007238083120696",
        // next round
        "23947619952183462858644581465494050309407721428302029371055887418452994318961",
        "25035254658153233628169609451068923631269927394392748023889572264723092874720",
        "17468020412163678868776493601957969748197290347006692843306595815987772942732",
        // next round
        "15262198027618900223004625662874755104828479630165814039838611768431063172994",
        "25161066724266754383358798644805908588326959881061318668106454787543611445887",
        "2454250001039770891411267760383268680504653332090622148533496270387793031332",
        // next round
        "9171946491887082474979985164918822959719377078284664312866368737511724712644",
        "6672870238005411132577302023934139592378291207852994424857452575898007687159",
        "2950400608762766076731526167833938554190979516192019010641815746350334547745",
        // next round
        "10653725154501691589476837895400001173933804810435931645261606197625601363132",
        "12717400214508961810851553873706609743505640660238109459222577386574996883747",
        "5871058785976817081042949511195036111847495052209270758342334312740290470200",
        // next round
        "18192562665205900830717234913238180302424621739145466326708104656354353538015",
        "19946412409172091711185698839696950657650658896270607012902209489827790455314",
        "21997416257528392077410699901606794827305154904508120972585193876767785262539",
        // next round
        "16525092684784199198745517563091041705366544303388462641935777835264970071331",
        "27613372589672512522307803997948488817865025374001297632527692577079750053456",
        "23369674747888778238616865774843237791546925005553032792584302158017141634655",
        // next round
        "11012136308159330675912474383855146192700147583104742924419195363346115019405",
        "20632243971343595216801828590185617698839041744000918292113739726624680548813",
        "10530371852841765918702282883445676639977895775479854136871270050807595649710",
        // next round
        "1610594053831245596683250788274018471388810111366046583216577135605955718023",
        "452300846172044702598793611907955884294868639769163388132276731316720796255",
        "22297945145153422883128810575530182077542612397826351322358420927950400316504",
        // next round
        "28212510899948152845929142163236606049756849316851154583029383581129293825706",
        "28325924586146971645663587791728624896861517146549428987043066595915712075981",
        "23489013325315178311518261165509151135555509351661386106070231815049642443022",
        // next round
        "10150108696154604591036176090028652090941375062280095655463112192524823306544",
        "14935856239824547404885450872472169780177654619496758596151670953532153419587",
        "4367251608666794961207658726914177158125339342277880902441218521648798930454",
        // next round
        "14278046449956534912766622635951826857049583276976844525135170835571509013020",
        "11627801940273881243235293875277734806211947530882079339115454640100174268255",
        "22853853581419894582873479603685652928885253184240650995805892818180355600894",
        // next round
        "4405193089432137585625363585733613667088817369599257533888439029942466720878",
        "26434497741746827048559732407319982377645052620918789373329661707603241810667",
        "23558650878002025381506445692526977061352711282820117441110868042756853707843",
        // next round
        "27427423077748345654234924309581695092179468167973406115643356520054395647078",
        "17585801825757985265979208086560185342609289319992678737491966299829354657891",
        "22079131836316223121286612953926945430480043835170303484162677394496378207190",
        // next round
        "20126865597655889981803452476686954944892814234259869552204215672627920656068",
        "5591585339015997308682985123056479221565470335707041924016523106405300562835",
        "9422316572086279209843572429137982927615080330725918371521370800874341571474",
        // next round
        "2735677349719528139570614238939713941030373684882307164259316901880218894412",
        "16229147459127626384090303399894157248853232127961182470501666316464149067069",
        "17151067888069760812629817914442472623785916486309268828873486698948911058517",
        // next round
        "13833972862865550568348750465964022581895521701070662509936215512761615491351",
        "9624679817699048440664645568701817641311119158936258215534754849666144699339",
        "10273179847163882031630140477902608240997857384703412878925192706057610103613",
        // next round
        "3172037826021850467928085880043492158321918352296515787555947245998877188849",
        "28890802281119993101506497911757988639840653958256859430239635494708187190915",
        "23496953773368274731821824281559682992786773767847557735733251263969009271239",
        // next round
        "1509044982655321910215442389040863370827049078919961070795919190828975736187",
        "13927172650979098916742472053302036482743492746437467103459483008024082210879",
        "17248379591027039069313293591621091031164062825086122980769287846951363066520",
        // next round
        "11350333545134487336540967650634077894516131586708748380417042089147896079201",
        "639497848254405996993150855123515463224731962182127668267769103213580096582",
        "24528361599642320451530127347946798949257664936307333999618279589325586618880",
        // next round
        "8217015496508457685301448884203977810298711070026260090660268003968421268717",
        "6703444480721420507060701216472376128524677965704475494357937059812166295103",
        "8051365375874262471960241848873604339195556527603956582828833313772444122472",
        // next round
        "10412735174026641936105532807659667596947675372330827493649954160029449767122",
        "8447576362386697729021229138353952824970707645851763166490398451107606293885",
        "4802965296970904162106502573136505305073730277702271660292532219583823320181",
        // next round
        "3244354881334856885788568976540712586633556478250043997221528214026130052269",
        "817270901440592571623549787267103386561304980129799240746702119063425010300",
        "6566338353152134577893356938981496347522747926131278635019050445923229718029",
        // next round
        "4854521709622003124815206874897232905514824969466266873443062691298769768277",
        "12830134034124699064152980183243986699241944691238427861184919962819448276943",
        "24309439157688106320977023683093060719537142150089588950480669629964661236785",
        // next round
        "1853791709949511636795588377016980571084333441972847324139062389997895453872",
        "11399505004623970417786749745036397690793259153591025248188283534764565207306",
        "6280235834578097246976697944083887557501831809932305676532914637669922657807",
        // next round
        "1516294190187225192808636261678393666537186816904214776860202535671714230097",
        "5835813607391397757416951433662507638966861369364000865214031356023042341328",
        "25777313996516799380163546628133415256678997511953860435781885414872422583905",
        // next round
        "9749298878960864917089442034293906589697892682402070689770627645324414273893",
        "19986612197193695239708718365565978831607994386509967951279410162135133793419",
        "5020585421647265067890838871263925730422335215511670656851726444447972642755",
        // next round
        "7256822974971238434100017358319972368738353570339258522235883585691301791128",
        "9789139064283320903202623693175751994730652446378861671859478926598420184293",
        "19283468246375057076525422714896652730563534118070235174488237489890270899533",
        // next round
        "11487321478704551489982188818171823402443882145686911658585221913500937481156",
        "16513958012405406860890342996091255867910990589443610357743227675107758695101",
        "24764429351173766080138047602436205744310671344674490826288279531917797263231",
        // next round
        "8256258316375000496541664568891934707113720493937218096466691600593595285909",
        "26919625894863883593081175799908601863265420311251948374988589188905317081443",
        "10135851848127171199130812615581006825969108287418884763125596866448544567342",
        // next round
        "17567146349912867622479843655652582453162587996421871126612027345809646551661",
        "2524802431860351616270075327416865184018211992251290134350377936184047953453",
        "3417609143162661859785838333493682460709943782149216513733553607075915176256",
        // next round
        "6906455011502599710165862205505812668908382042647994457156780865092846286493",
        "21042097659487317081899343674473811663642293019125869396575405454328274948985",
        "25222370053690749913129090298406788520061040938312366403907461864202905656238",
        // next round
        "18933201791079410639949505893100361911334261775545573219434897335758052335005",
        "14503331557348715387048413780116585195932777696828173626366829282421027153184",
        "3558781473325529402549318082942465709639711182863041375748599816583729962116",
        // next round
        "23932570601084008621895097434501731960424360312878373523779451810455362953625",
        "13286131463754478912858022007443470896920464302917391606059553157137090717219",
        "9969435194445819847988134248075866286921574284754991873902788928171429847506",
        // next round
        "10821551500865029673311799086099720530496516676117927814621168667836737594374",
        "57689402905128519605376551862931564078571458212398163192591670282543962941",
        "4484359679395800410695081358212522306960518636189521201445105538223906998486",
    ],
};