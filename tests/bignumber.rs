/*
    Copyright Michael Lodder. All Rights Reserved.
    SPDX-License-Identifier: Apache-2.0
*/
use digest::Digest;
use unknown_order::*;

/// Taken from https://github.com/mikelodder7/cunningham_chain/blob/master/findings.md
/// prefix'd with '9' for multibase to work
const TEST_PRIMES: [&str; 4] = [
    "9153739637779647327330155094463476939112913405723627932550795546376536722298275674187199768137486929460478138431076223176750734095693166283451594721829574797878338183845296809008576378039501400850628591798770214582527154641716248943964626446190042367043984306973709604255015629102866732543697075866901827761489",
    "966295144163396665403376179086308918015255210762161712943347745256800426733181435998953954369657699924569095498869393378860769817738689910466139513014839505675023358799693196331874626976637176000078613744447569887988972970496824235261568439949705345174465781244618912962800788579976795988724553365066910412859",
    "937313426856874901938110133384605074194791927500210707276948918975046371522830901596065044944558427864187196889881993164303255749681644627614963632713725183364319410825898054225147061624559894980555489070322738683900143562848200257354774040241218537613789091499134051387344396560066242901217378861764936185029",
    "989884656743115795386465259539451236680898848947115328636715040578866337902750481566354238661203768010560056939935696678829394884407208311246423715319737062188883946712432742638151109800623047059726541476042502884419075341171231440736956555270413618581675255342293149119973622969239858152417678164815053566739"
];

fn get_modulus() -> BigNumber {
    b10(TEST_PRIMES[0]) * b10(TEST_PRIMES[1])
}

/// prefix with 9 any input
fn b10(s: &str) -> BigNumber {
    let (_, bytes) = multibase::decode(s).unwrap();
    BigNumber::from_slice(bytes.as_slice())
}

#[test]
fn random() {
    let n = get_modulus();
    for _ in 0..100 {
        let s = BigNumber::random(&n);
        assert!(s < n);
    }
}

#[test]
fn hash() {
    let mut hasher = blake2::Blake2b::new();
    hasher.update(b"an arbitrary sequence of bytes");
    let s = BigNumber::from_digest(hasher);
    assert!(!s.is_zero())
}

#[test]
fn invert() {
    let n = get_modulus();
    let seven = BigNumber::from(7);
    let res = seven.invert(&n);
    assert!(res.is_some());
    let inv_sev = res.unwrap();
    let e = b10("98736164100197231989787188588600960668069231385527654883722188521294636032401969483008945072483969138624775854861975726576062103939220928630158097991729478054488847175819214957276712990801597205987508160592161411562878226113426472758518077060830360520857340372917204559754499877424661206747919186595155095664390759910479790933107207818246310062809031809548440757639655172156206658836643666598028545699906946474098999286204150351756528088230861166258151032711654628115284610488946624661733330727293087598638805428569835503052197782968695111929188140960550805397118405320674665165150825485362977018330562195374374788901");
    assert_eq!(e, inv_sev);

    let a = BigNumber::random(&n);
    let res = a.invert(&n);
    assert!(res.is_some());
}

#[test]
#[should_panic]
fn bad_exp() {
    let base = BigNumber::from(7);
    let exp = BigNumber::from(3);
    let modulus = BigNumber::from(0);
    base.modpow(&exp, &modulus);
}

#[test]
fn exp() {
    // known bad inputs
    let base = b10("912714671911903680502393098440562958150461307840092575886187217264492970515611166458444182780904860535776274190597528985988632488194981204988199325501696648896748368401254829974173258613724800116424602180755019588176641580062215499750550535543002990347313784260314641340394494547935943176226649412526659864646068220114536172189443925908781755710141006387091748541976715633668919725277837668568166444731358541327097786024076841158424402136565558677098853060675674958695935207345864359540948421232816012865873346545455513695413921957708811080877422273777355768568166638843699798663264533662595755767287970642902713301649");
    let exp = b10("913991423645225256679625502829143442357836305738777175327623021076136862973228390317258480888217725740262243618881809894688804251512223982403225288178492105393953431042196371492402144120299046493467608097411259757604892535967240041988260332063962457178993277482991886508015739613530825229685281072180891075265116698114782553748364913010741387964956740720544998915158970813171997488129859542399633104746793770216517872705889857552727967921847493285577238");
    let modulus = b10("9991272771610724400277702356109350334773782112020672787325464582894874455338156617087078683660308327009158085342465983713825070967004447592080649030930737560915527173820649490032274245863850782844569456999473516497618489127293328524608584652323593452247534656999363158875176879817952982494174728640545484193154314433925648566686738628413929222467005197087738850212963801663981588243042912430590088435419451359859770426041670326127890520192033283832465411962274045956439947646966560440910244870464709982605844468449227905039953511431640780483761563845223213570597106855699997837768334871601402132694515676785338799407204529154456178837013845488372635042715003769626150545960460800980936426723680755798495767188398126674428244764038147226578038085253616108968402209263400729503458144370189359160926796812468410806201905992347006546335038212090539118675048292666041345556742530041533878341459110515497642054583635133581316796089099043782055893003258788369004899742992039315008110063759802733045648131896557338576682560236591353394201381103042167106112201578883917022695113857967398885475101031596068885337186646296664517159150904935112836318654117577507707562065113238913343761942585545093919444150946120523831367132144754209388110483749");
    let n = base.modpow(&exp, &modulus);
    assert_eq!(n, b10("9156669382818249607878298589043381544147555658222157929549484054385620519150887267126359684884641035264854247223281407349108771361611707714806192334779156374961296686821846487267487447347213829476609283133961216115764596907219173912888367998704856300105745961091899745329082513615681466199188236178266479183520370119131067362815102553237342546358580424556049196548520326206809677290296313839918774603549816182657993044271509706055893922152644469350618465711055733369291523796837304622919600074130968607301641438272377350795631212741686475924538423333008944556761300787668873766797549942827958501053262330421256183088509761636226277739400954175538503984519144969688787730088704522060486181427528150632576628856946041322195818246199503927686629821338146828603690778689292695518745939007886131151503766930229761608131819298276772877945842806872426029069949874062579870088710097070526608376602732627661781899595747063793310401032556802468649888104062151213860356554306295111191704764944574687548637446778783560586599000631975868701382113259027374431129732911012887214749014288413818636520182416636289308770657630129067046301651835893708731812616847614495049523221056260334965662875649480493232265453415256612460815802528012166114764216881"));

    let base = BigNumber::from(6);
    let exp = BigNumber::from(-5);
    let modulus = BigNumber::from(13);
    assert_eq!(BigNumber::from(7), base.modpow(&exp, &modulus));

    let modulus = BigNumber::from(1);
    assert_eq!(BigNumber::zero(), base.modpow(&exp, &modulus));

    let modulus = BigNumber::from(-1);
    assert_eq!(BigNumber::default(), base.modpow(&exp, &modulus));

    let modulus = BigNumber::from(-5);
    assert_eq!(BigNumber::from(1), base.modpow(&exp, &modulus));
}

#[test]
fn modulus() {
    let base = BigNumber::from(6);

    for (modulus, expected) in [
        (BigNumber::from(1), BigNumber::zero()),
        (BigNumber::from(-1), BigNumber::zero()),
        (BigNumber::from(2), BigNumber::zero()),
        (BigNumber::from(-2), BigNumber::zero()),
        (BigNumber::from(5), BigNumber::from(1)),
        (BigNumber::from(-5), BigNumber::from(1)),
    ]
    .iter()
    {
        assert_eq!(*expected, &base % modulus);
    }
}

#[test]
fn is_prime() {
    // taken from https://github.com/mikelodder7/cunningham_chain/blob/master/findings.md
    let tests =
        [("918088387217903330459", 6),
            ("933376463607021642560387296949", 6),
            ("9170141183460469231731687303717167733089", 6),
            ("9113910913923300788319699387848674650656041243163866388656000063249848353322899", 5),
            ("91675975991242824637446753124775730765934920727574049172215445180465220503759193372100234287270862928461253982273310756356719235351493321243304213304923049", 5),
            ("9153739637779647327330155094463476939112913405723627932550795546376536722298275674187199768137486929460478138431076223176750734095693166283451594721829574797878338183845296809008576378039501400850628591798770214582527154641716248943964626446190042367043984306973709604255015629102866732543697075866901827761489", 4),
            ("966295144163396665403376179086308918015255210762161712943347745256800426733181435998953954369657699924569095498869393378860769817738689910466139513014839505675023358799693196331874626976637176000078613744447569887988972970496824235261568439949705345174465781244618912962800788579976795988724553365066910412859", 4),
        ];

    let one = BigNumber::from(1);
    for (p, chain) in tests.iter() {
        let mut prime = b10(*p);
        for _ in 1..*chain {
            prime = (prime << 1usize) + &one;
            assert!(prime.is_prime());
        }
    }
}

#[test]
fn clone_negative() {
    let n = BigNumber::from(-1);
    assert_eq!(n, n.clone());
}

#[test]
fn serialize() {
    let n = b10(TEST_PRIMES[2]);
    let res = bincode::serialize(&n);
    assert!(res.is_ok());
    let s = res.unwrap();
    let nn_res = bincode::deserialize::<BigNumber>(&s);
    assert!(nn_res.is_ok());
    assert_eq!(nn_res.unwrap(), n);

    let n = -BigNumber::from(1);
    let res = bincode::serialize(&n);
    assert!(res.is_ok());
    let s = res.unwrap();
    let nn_res = bincode::deserialize::<BigNumber>(&s);
    assert!(nn_res.is_ok());
    assert_eq!(-nn_res.unwrap(), n);

    // bincode uses 8 bytes for the length of the vector
    assert!(bincode::deserialize::<BigNumber>(&[1, 0, 0, 0, 0, 0, 0, 0, 1]).is_ok())
}

#[test]
fn prime() {
    let p = BigNumber::prime(1024);
    assert!(p.is_prime());
    let s = p.to_string().len();
    assert!(s <= 309);
}

#[test]
fn safe_prime() {
    // any larger and it will take a long time
    let p = BigNumber::safe_prime(256);
    assert!(p.is_prime());
    let ptick: BigNumber = p >> 1;
    assert!(ptick.is_prime());
}

#[test]
fn gcd_ext() {
    let a = BigNumber::from(13);
    let b = BigNumber::from(17);
    let res = a.extended_gcd(&b);
    assert_eq!(res.gcd, BigNumber::one());
}

#[test]
fn bytes() {
    let m = BigNumber::from(7);
    let s = m.to_bytes();
    assert_eq!(m, BigNumber::from_slice(&s));
}
