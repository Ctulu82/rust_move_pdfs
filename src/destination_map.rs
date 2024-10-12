use std::collections::HashMap;

// 공통 경로를 상수로 선언
const BASE_PATH: &str = "./Downloads/_DONE/_Magazines/";

pub fn get_destination_map() -> HashMap<String, String> {
    let mut destination_map: HashMap<String, String> = HashMap::new();

    // format!을 사용하여 각 경로를 문자열로 생성
    // TODO: 이름이 겹치지 않도록 확인! & 잡지사 별로 묶을지 ABC순서대로 나열할지 고민 필요.

	// 5280 Publishing
	destination_map.insert(
		"5280 Magazine ".to_string(),
		format!("{}__5280 Publishing/5280 Magazine", BASE_PATH),
	);
	
	// AAAS
	destination_map.insert(
		"Science 20".to_string(),
		format!("{}__AAAS/Science", BASE_PATH),
	);

	// AAS Sky Publishing
	destination_map.insert(
		"Australian Sky & Telescope ".to_string(),
		format!("{}__AAS Sky Publishing/Australian Sky & Telescope", BASE_PATH),
	);
	destination_map.insert(
		"Sky & Telescope ".to_string(),
		format!("{}__AAS Sky Publishing/Sky & Telescope", BASE_PATH),
	);

	// __ABK Publications
	destination_map.insert(
		"Australian Birdkeeper ".to_string(),
		format!("{}__ABK Publications/Australian Birdkeeper", BASE_PATH),
	);

	// Active Interest Media
	destination_map.insert(
		"Anglers Journal ".to_string(),
		format!("{}__Active Interest Media/Anglers Journal", BASE_PATH),
	);
	destination_map.insert(
		"Bank Note Reporter ".to_string(),
		format!("{}__Active Interest Media/Bank Note Reporter", BASE_PATH),
	);
	destination_map.insert(
		"Cozy Cabins & Cottages ".to_string(),
		format!("{}__Active Interest Media/Cozy Cabins & Cottages", BASE_PATH),
	);
	destination_map.insert(
		"Cuisine at Home ".to_string(),
		format!("{}__Active Interest Media/Cuisine at Home", BASE_PATH),
	);
	destination_map.insert(
		"Fine Gardening ".to_string(),
		format!("{}__Active Interest Media/Fine Gardening", BASE_PATH),
	);
	destination_map.insert(
		"Fine Homebuilding ".to_string(),
		format!("{}__Active Interest Media/Fine Homebuilding", BASE_PATH),
	);
	destination_map.insert(
		"Kovels Antique Trader ".to_string(),
		format!("{}__Active Interest Media/Kovels Antique Trader", BASE_PATH),
	);
	destination_map.insert(
		"Log & Timber ".to_string(),
		format!("{}__Active Interest Media/Log & Timber", BASE_PATH),
	);
	destination_map.insert(
		"Numismatic News ".to_string(),
		format!("{}__Active Interest Media/Numismatic News", BASE_PATH),
	);
	destination_map.insert(
		"Old House Journal ".to_string(),
		format!("{}__Active Interest Media/Old House Journal", BASE_PATH),
	);
	destination_map.insert(
		"PassageMaker ".to_string(),
		format!("{}__Active Interest Media/PassageMaker", BASE_PATH),
	);
	destination_map.insert(
		"Sail ".to_string(),
		format!("{}__Active Interest Media/Sail", BASE_PATH),
	);
	destination_map.insert(
		"Soundings ".to_string(),
		format!("{}__Active Interest Media/Soundings", BASE_PATH),
	);
	destination_map.insert(
		"World Coin News ".to_string(),
		format!("{}__Active Interest Media/World Coin News", BASE_PATH),
	);
	destination_map.insert(
		"Writer's Digest ".to_string(),
		format!("{}__Active Interest Media/Writer's Digest", BASE_PATH),
	);
	destination_map.insert(
		"Yachts International ".to_string(),
		format!("{}__Active Interest Media/Yachts International", BASE_PATH),
	);

	// AFV Modeller
	destination_map.insert(
		"AFV Modeller ".to_string(),
		format!("{}__AFV Modeller/AFV Modeller", BASE_PATH),
	);
	destination_map.insert(
		"Air Modeller ".to_string(),
		format!("{}__AFV Modeller/Air Modeller", BASE_PATH),
	);

	// AJ Bell Media
	destination_map.insert(
		"Shares ".to_string(),
		format!("{}__AJ Bell Media/Shares", BASE_PATH),
	);

	// America’s Test Kitchen
	destination_map.insert(
		"Cook's Country ".to_string(),
		format!("{}__America’s Test Kitchen/Cook's Country", BASE_PATH),
	);
	destination_map.insert(
		"Cook's Illustrated ".to_string(),
		format!("{}__America’s Test Kitchen/Cook's Illustrated", BASE_PATH),
	);

	// Anthem
	destination_map.insert(
		"Air Fryer Cookbook ".to_string(),
		format!("{}__Anthem/Air Fryer Cookbook", BASE_PATH),
	);
	destination_map.insert(
		"Classic Pop ".to_string(),
		format!("{}__Anthem/Classic Pop", BASE_PATH),
	);
	destination_map.insert(
		"Italia! ".to_string(),
		format!("{}__Anthem/Italia!", BASE_PATH),
	);
	destination_map.insert(
		"Vegan Food & Living ".to_string(),
		format!("{}__Anthem/Vegan Food & Living", BASE_PATH),
	);
	destination_map.insert(
		"Vintage Rock ".to_string(),
		format!("{}__Anthem/Vintage Rock", BASE_PATH),
	);

	// Archaeological Institute of America
	destination_map.insert(
		"Archaeology 2".to_string(),
		format!("{}__Archaeological Institute of America/Archaeology", BASE_PATH),
	);

	// Architecture Media
	destination_map.insert(
		"Architectural Product News ".to_string(),
		format!("{}__Architecture Media/Architectural Product News", BASE_PATH),
	);
	destination_map.insert(
		"Architecture Australia ".to_string(),
		format!("{}__Architecture Media/Architecture Australia", BASE_PATH),
	);
	destination_map.insert(
		"Artichoke ".to_string(),
		format!("{}__Architecture Media/Artichoke", BASE_PATH),
	);
	destination_map.insert(
		"Houses AU ".to_string(),
		format!("{}__Architecture Media/Houses AU", BASE_PATH),
	);
	destination_map.insert(
		"Landscape Architecture AU ".to_string(),
		format!("{}__Architecture Media/Landscape Architecture AU", BASE_PATH),
	);

	// Are Media
	destination_map.insert(
		"Australian Gourmet Traveller ".to_string(),
		format!("{}__Are Media/Australian Gourmet Traveller", BASE_PATH),
	);
	destination_map.insert(
		"Belle ".to_string(),
		format!("{}__Are Media/Belle", BASE_PATH),
	);
	destination_map.insert(
		"Better Homes and Gardens AU ".to_string(),
		format!("{}__Are Media/Better Homes and Gardens AU", BASE_PATH),
	);
	destination_map.insert(
		"Cooking With The Australian Woman's Weekly ".to_string(),
		format!("{}__Are Media/Cooking With The Australian Woman's Weekly", BASE_PATH),
	);
	destination_map.insert(
		"Country Style ".to_string(),
		format!("{}__Are Media/Country Style", BASE_PATH),
	);
	destination_map.insert(
		"Diabetic Living ".to_string(),
		format!("{}__Are Media/Diabetic Living", BASE_PATH),
	);
	destination_map.insert(
		"Home Beautiful AU ".to_string(),
		format!("{}__Are Media/Home Beautiful AU", BASE_PATH),
	);
	destination_map.insert(
		"House & Garden AU ".to_string(),
		format!("{}__Are Media/House & Garden AU", BASE_PATH),
	);
	destination_map.insert(
		"Inside Out ".to_string(),
		format!("{}__Are Media/Inside Out", BASE_PATH),
	);
	destination_map.insert(
		"New Idea Food ".to_string(),
		format!("{}__Are Media/New Idea Food", BASE_PATH),
	);
	destination_map.insert(
		"The Australian Woman's Weekly ".to_string(),
		format!("{}__Are Media/The Australian Woman's Weekly", BASE_PATH),
	);
	destination_map.insert(
		"Your Home and Garden ".to_string(),
		format!("{}__Are Media/Your Home and Garden", BASE_PATH),
	);

	// Artichoke
	destination_map.insert(
		"Great British Food ".to_string(),
		format!("{}__Artichoke/Great British Food", BASE_PATH),
	);
	destination_map.insert(
		"Grow Your Own ".to_string(),
		format!("{}__Artichoke/Grow Your Own", BASE_PATH),
	);
	destination_map.insert(
		"Teach Primary ".to_string(),
		format!("{}__Artichoke/Teach Primary", BASE_PATH),
	);
	destination_map.insert(
		"Teach Reading & Writing ".to_string(),
		format!("{}__Artichoke/Teach Reading & Writing", BASE_PATH),
	);
	destination_map.insert(
		"Teach Secondary ".to_string(),
		format!("{}__Artichoke/Teach Secondary", BASE_PATH),
	);

	// Ashdown
	destination_map.insert(
		"Bead & Jewellery ".to_string(),
		format!("{}__Ashdown/Bead & Jewellery", BASE_PATH),
	);

	// BizClik Media
	destination_map.insert(
		"AI Magazine ".to_string(),
		format!("{}__BizClik Media/AI Magazine", BASE_PATH),
	);
	destination_map.insert(
		"Cyber Magazine ".to_string(),
		format!("{}__BizClik Media/Cyber Magazine", BASE_PATH),
	);
	destination_map.insert(
		"Technology 2".to_string(),
		format!("{}__BizClik Media/Technology", BASE_PATH),
	);

	// Bloomberg
	destination_map.insert(
		"Bloomberg Businessweek ".to_string(),
		format!("{}__Bloomberg/Bloomberg Businessweek", BASE_PATH),
	);

	// BNP Media
	destination_map.insert(
		"Architectural Record ".to_string(),
		format!("{}__BNP Media/Architectural Record", BASE_PATH),
	);

	// Bonnier
	destination_map.insert(
		"Anti-Inflammatory ".to_string(),
		format!("{}__Bonnier/Anti-Inflammatory", BASE_PATH),
	);
	destination_map.insert(
		"Boating ".to_string(),
		format!("{}__Bonnier/Boating", BASE_PATH),
	);
	destination_map.insert(
		"Bringing History to Life ".to_string(),
		format!("{}__Bonnier/Bringing History to Life", BASE_PATH),
	);
	destination_map.insert(
		"Cruising World ".to_string(),
		format!("{}__Bonnier/Cruising World", BASE_PATH),
	);
	destination_map.insert(
		"Inside History Collection ".to_string(),
		format!("{}__Bonnier/Inside History Collection", BASE_PATH),
	);
	destination_map.insert(
		"Sailing World ".to_string(),
		format!("{}__Bonnier/Sailing World", BASE_PATH),
	);
	destination_map.insert(
		"Salt Water Sportsman ".to_string(),
		format!("{}__Bonnier/Salt Water Sportsman", BASE_PATH),
	);
	destination_map.insert(
		"Yachting ".to_string(),
		format!("{}__Bonnier/Yachting", BASE_PATH),
	);

	// Canada Wide Media
	destination_map.insert(
		"Western Living ".to_string(),
		format!("{}__Canada Wide Media/Western Living", BASE_PATH),
	);

	// Castle Media
	destination_map.insert(
		"Build It ".to_string(),
		format!("{}__Castle Media/Build It", BASE_PATH),
	);

	// Challenge Publications
	destination_map.insert(
		"Air Classics ".to_string(),
		format!("{}__Challenge Publications/Air Classics", BASE_PATH),
	);

	// China News Service
	destination_map.insert(
		"China Report ".to_string(),
		format!("{}__China News Service/China Report", BASE_PATH),
	);

	// Condé Nast
	destination_map.insert(
		"Architectural Digest ".to_string(),
		format!("{}__Condé Nast/Architectural Digest", BASE_PATH),
	);
	destination_map.insert(
		"Bon Appétit ".to_string(),
		format!("{}__Condé Nast/Bon Appétit", BASE_PATH),
	);
	destination_map.insert(
		"Condé Nast Traveler ".to_string(),
		format!("{}__Condé Nast/Condé Nast Traveler", BASE_PATH),
	);
	destination_map.insert(
		"Condé Nast Traveller ".to_string(),
		format!("{}__Condé Nast/Condé Nast Traveller", BASE_PATH),
	);
	destination_map.insert(
		"Tatler ".to_string(),
		format!("{}__Condé Nast/Tatler", BASE_PATH),
	);
	destination_map.insert(
		"The New Yorker ".to_string(),
		format!("{}__Condé Nast/The New Yorker", BASE_PATH),
	);
	destination_map.insert(
		"The World of Interiors ".to_string(),
		format!("{}__Condé Nast/The World of Interiors", BASE_PATH),
	);
	destination_map.insert(
		"Wired 20".to_string(),
		format!("{}__Condé Nast/Wired", BASE_PATH),
	);
	destination_map.insert(
		"Wired ME ".to_string(),
		format!("{}__Condé Nast/Wired ME", BASE_PATH),
	);
	destination_map.insert(
		"Wired UK ".to_string(),
		format!("{}__Condé Nast/Wired UK", BASE_PATH),
	);

	// DC Thompson
	destination_map.insert(
		"110% Gaming ".to_string(),
		format!("{}__DC Thompson/110% Gaming", BASE_PATH),
	);
	destination_map.insert(
		"Beano ".to_string(),
		format!("{}__DC Thompson/Beano", BASE_PATH),
	);
	destination_map.insert(
		"Commando ".to_string(),
		format!("{}__DC Thompson/Commando", BASE_PATH),
	);
	destination_map.insert(
		"The Scots Magazine ".to_string(),
		format!("{}__DC Thompson/The Scots Magazine", BASE_PATH),
	);
	destination_map.insert(
		"This England ".to_string(),
		format!("{}__DC Thompson/This England", BASE_PATH),
	);

	// Diamond Publishing
	destination_map.insert(
		"Best of British ".to_string(),
		format!("{}__Diamond Publishing/Best of British", BASE_PATH),
	);
	destination_map.insert(
		"Fortean Times ".to_string(),
		format!("{}__Diamond Publishing/Fortean Times", BASE_PATH),
	);
	destination_map.insert(
		"Record Collector ".to_string(),
		format!("{}__Diamond Publishing/Record Collector", BASE_PATH),
	);

	// Doolittle Media
	destination_map.insert(
		"Military Illustrated Modeller ".to_string(),
		format!("{}__Doolittle Media/Military Illustrated Modeller", BASE_PATH),
	);
	destination_map.insert(
		"Model Airplane International ".to_string(),
		format!("{}__Doolittle Media/Model Airplane International", BASE_PATH),
	);
	destination_map.insert(
		"Model Military International ".to_string(),
		format!("{}__Doolittle Media/Model Military International", BASE_PATH),
	);
	destination_map.insert(
		"Tamiya Model Magazine International ".to_string(),
		format!("{}__Doolittle Media/Tamiya Model Magazine International", BASE_PATH),
	);

	// Dow Jones & Company
	destination_map.insert(
		"Barron's ".to_string(),
		format!("{}__Dow Jones & Company/Barron's", BASE_PATH),
	);

	// EG Media
	destination_map.insert(
		"American Farmhouse Style ".to_string(),
		format!("{}__EG Media/American Farmhouse Style", BASE_PATH),
	);
	destination_map.insert(
		"Atomic Ranch ".to_string(),
		format!("{}__EG Media/Atomic Ranch", BASE_PATH),
	);
	destination_map.insert(
		"COINage ".to_string(),
		format!("{}__EG Media/COINage", BASE_PATH),
	);
	destination_map.insert(
		"Chickens ".to_string(),
		format!("{}__EG Media/Chickens", BASE_PATH),
	);
	destination_map.insert(
		"Cottages & Bungalows ".to_string(),
		format!("{}__EG Media/Cottages & Bungalows", BASE_PATH),
	);
	destination_map.insert(
		"Hobby Farms ".to_string(),
		format!("{}__EG Media/Hobby Farms", BASE_PATH),
	);
	destination_map.insert(
		"Rock & Gem ".to_string(),
		format!("{}__EG Media/Rock & Gem", BASE_PATH),
	);

	// Emerald X
	destination_map.insert(
		"Hospitality Design ".to_string(),
		format!("{}__Emerald X/Hospitality Design", BASE_PATH),
	);
	destination_map.insert(
		"Kitchen & Bath Business ".to_string(),
		format!("{}__Emerald X/Kitchen & Bath Business", BASE_PATH),
	);
	destination_map.insert(
		"Pizza Today ".to_string(),
		format!("{}__Emerald X/Pizza Today", BASE_PATH),
	);

	// Fusion Retro Books
	destination_map.insert(
		"AmtixCPC ".to_string(),
		format!("{}__Fusion Retro Books/AmtixCPC", BASE_PATH),
	);
	destination_map.insert(
		"Crash #".to_string(),
		format!("{}__Fusion Retro Books/Crash", BASE_PATH),
	);
	destination_map.insert(
		"Fusion ".to_string(),
		format!("{}__Fusion Retro Books/Fusion", BASE_PATH),
	);
	destination_map.insert(
		"ZZAP! 64 ".to_string(),
		format!("{}__Fusion Retro Books/ZZAP! 64", BASE_PATH),
	);

    // Future Plc
	destination_map.insert(
		"25 Beautiful Homes ".to_string(),
		format!("{}__Future Plc/25 Beautiful Homes", BASE_PATH),
	);
	destination_map.insert(
		"3D World ".to_string(),
		format!("{}__Future Plc/3D World", BASE_PATH),
	);
	destination_map.insert(
		"APC ".to_string(),
		format!("{}__Future Plc/APC", BASE_PATH),
	);
	destination_map.insert(
		"All About History ".to_string(),
		format!("{}__Future Plc/All About History", BASE_PATH),
	);
	destination_map.insert(
		"All About Space ".to_string(),
		format!("{}__Future Plc/All About Space", BASE_PATH),
	);
	destination_map.insert(
		"Amateur Gardening ".to_string(),
		format!("{}__Future Plc/Amateur Gardening", BASE_PATH),
	);
	destination_map.insert(
		"Classic Rock ".to_string(),
		format!("{}__Future Plc/Classic Rock", BASE_PATH),
	);
	destination_map.insert(
		"Computer Music ".to_string(),
		format!("{}__Future Plc/Computer Music", BASE_PATH),
	);
	destination_map.insert(
		"Computeractive ".to_string(),
		format!("{}__Future Plc/Computeractive", BASE_PATH),
	);
	destination_map.insert(
		"Country Home & Interiors ".to_string(),
		format!("{}__Future Plc/Country Home & Interiors", BASE_PATH),
	);
	destination_map.insert(
		"Country Life ".to_string(),
		format!("{}__Future Plc/Country Life", BASE_PATH),
	);
	destination_map.insert(
		"Edge #".to_string(),
		format!("{}__Future Plc/Edge", BASE_PATH),
	);
	destination_map.insert(
		"FourFourTwo ".to_string(),
		format!("{}__Future Plc/FourFourTwo", BASE_PATH),
	);
	destination_map.insert(
		"Future Music ".to_string(),
		format!("{}__Future Plc/Future Music", BASE_PATH),
	);
	destination_map.insert(
		"Guitar Player ".to_string(),
		format!("{}__Future Plc/Guitar Player", BASE_PATH),
	);
	destination_map.insert(
		"Guitar Techniques ".to_string(),
		format!("{}__Future Plc/Guitar Techniques", BASE_PATH),
	);
	destination_map.insert(
		"Guitar World ".to_string(),
		format!("{}__Future Plc/Guitar World", BASE_PATH),
	);
	destination_map.insert(
		"Guitarist ".to_string(),
		format!("{}__Future Plc/Guitarist", BASE_PATH),
	);
	destination_map.insert(
		"History of War ".to_string(),
		format!("{}__Future Plc/History of War", BASE_PATH),
	);
	destination_map.insert(
		"Homebuilding & Renovating ".to_string(),
		format!("{}__Future Plc/Homebuilding & Renovating", BASE_PATH),
	);
	destination_map.insert(
		"Homes & Gardens ".to_string(),
		format!("{}__Future Plc/Homes & Gardens", BASE_PATH),
	);
	destination_map.insert(
		"How It Works ".to_string(),
		format!("{}__Future Plc/How It Works", BASE_PATH),
	);
	destination_map.insert(
		"Ideal Home ".to_string(),
		format!("{}__Future Plc/Ideal Home", BASE_PATH),
	);
	destination_map.insert(
		"ImagineFX ".to_string(),
		format!("{}__Future Plc/ImagineFX", BASE_PATH),
	);
	destination_map.insert(
		"iPad User Magazine ".to_string(),
		format!("{}__Future Plc/iPad User Magazine", BASE_PATH),
	);
	destination_map.insert(
		"Kiplinger's Personal Finance ".to_string(),
		format!("{}__Future Plc/Kiplinger's Personal Finance", BASE_PATH),
	);
	destination_map.insert(
		"Linux Format ".to_string(),
		format!("{}__Future Plc/Linux Format", BASE_PATH),
	);
	destination_map.insert(
		"Living Etc ".to_string(),
		format!("{}__Future Plc/Living Etc", BASE_PATH),
	);
	destination_map.insert(
		"Mac Format ".to_string(),
		format!("{}__Future Plc/Mac Format", BASE_PATH),
	);
	destination_map.insert(
		"Mac Life ".to_string(),
		format!("{}__Future Plc/Mac Life", BASE_PATH),
	);
	destination_map.insert(
		"Maximum PC ".to_string(),
		format!("{}__Future Plc/Maximum PC", BASE_PATH),
	);
	destination_map.insert(
		"Metal Hammer ".to_string(),
		format!("{}__Future Plc/Metal Hammer", BASE_PATH),
	);
	destination_map.insert(
		"Minecraft World Magazine ".to_string(),
		format!("{}__Future Plc/Minecraft World Magazine", BASE_PATH),
	);
	destination_map.insert(
		"MoneyWeek ".to_string(),
		format!("{}__Future Plc/MoneyWeek", BASE_PATH),
	);
	destination_map.insert(
		"Motor Boat & Yachting ".to_string(),
		format!("{}__Future Plc/Motor Boat & Yachting", BASE_PATH),
	);
	destination_map.insert(
		"Music Week ".to_string(),
		format!("{}__Future Plc/Music Week", BASE_PATH),
	);
	destination_map.insert(
		"PC Gamer UK ".to_string(),
		format!("{}__Future Plc/PC Gamer UK", BASE_PATH),
	);
	destination_map.insert(
		"PC Gamer US ".to_string(),
		format!("{}__Future Plc/PC Gamer US", BASE_PATH),
	);
	destination_map.insert(
		"PC Powerplay ".to_string(),
		format!("{}__Future Plc/PC Powerplay", BASE_PATH),
	);
	destination_map.insert(
		"PC Pro ".to_string(),
		format!("{}__Future Plc/PC Pro", BASE_PATH),
	);
	destination_map.insert(
		"PLAY #".to_string(),
		format!("{}__Future Plc/PLAY", BASE_PATH),
	);
	destination_map.insert(
		"PLAY AU ".to_string(),
		format!("{}__Future Plc/PLAY AU", BASE_PATH),
	);
	destination_map.insert(
		"Paint & Draw ".to_string(),
		format!("{}__Future Plc/Paint & Draw", BASE_PATH),
	);
	destination_map.insert(
		"Period Living ".to_string(),
		format!("{}__Future Plc/Period Living", BASE_PATH),
	);
	destination_map.insert(
		"Practical Boat Owner ".to_string(),
		format!("{}__Future Plc/Practical Boat Owner", BASE_PATH),
	);
	destination_map.insert(
		"Prog ".to_string(),
		format!("{}__Future Plc/Prog", BASE_PATH),
	);
	destination_map.insert(
		"Psychology Now ".to_string(),
		format!("{}__Future Plc/Psychology Now", BASE_PATH),
	);
	destination_map.insert(
		"Real Crime ".to_string(),
		format!("{}__Future Plc/Real Crime", BASE_PATH),
	);
	destination_map.insert(
		"Retro Gamer ".to_string(),
		format!("{}__Future Plc/Retro Gamer", BASE_PATH),
	);
	destination_map.insert(
		"SFX ".to_string(),
		format!("{}__Future Plc/SFX", BASE_PATH),
	);
	destination_map.insert(
		"Sound+Image ".to_string(),
		format!("{}__Future Plc/Sound+Image", BASE_PATH),
	);
	destination_map.insert(
		"Space.com ".to_string(),
		format!("{}__Future Plc/Space_com", BASE_PATH),
	);
	destination_map.insert(
		"Style at Home UK ".to_string(),
		format!("{}__Future Plc/Style at Home UK", BASE_PATH),
	);
	destination_map.insert(
		"T3 ".to_string(),
		format!("{}__Future Plc/T3", BASE_PATH),
	);
	destination_map.insert(
		"TV & Satellite Week ".to_string(),
		format!("{}__Future Plc/TV & Satellite Week", BASE_PATH),
	);
	destination_map.insert(
		"The Week Junior Science + Nature ".to_string(),
		format!("{}__Future Plc/The Week Junior Science + Nature", BASE_PATH),
	);
	destination_map.insert(
		"The Week Junior UK ".to_string(),
		format!("{}__Future Plc/The Week Junior UK", BASE_PATH),
	);
	destination_map.insert(
		"The Week Junior US ".to_string(),
		format!("{}__Future Plc/The Week Junior US", BASE_PATH),
	);
	destination_map.insert(
		"The Week UK ".to_string(),
		format!("{}__Future Plc/The Week UK", BASE_PATH),
	);
	destination_map.insert(
		"The Week US ".to_string(),
		format!("{}__Future Plc/The Week US", BASE_PATH),
	);
	destination_map.insert(
		"Total Film ".to_string(),
		format!("{}__Future Plc/Total Film", BASE_PATH),
	);
	destination_map.insert(
		"Total Guitar ".to_string(),
		format!("{}__Future Plc/Total Guitar", BASE_PATH),
	);
	destination_map.insert(
		"Wallpaper ".to_string(),
		format!("{}__Future Plc/Wallpaper", BASE_PATH),
	);
	destination_map.insert(
		"What Hi-Fi ".to_string(),
		format!("{}__Future Plc/What Hi-Fi", BASE_PATH),
	);
	destination_map.insert(
		"Woman & Home ".to_string(),
		format!("{}__Future Plc/Woman & Home", BASE_PATH),
	);
	destination_map.insert(
		"World of Animals ".to_string(),
		format!("{}__Future Plc/World of Animals", BASE_PATH),
	);
	destination_map.insert(
		"Yachting Monthly ".to_string(),
		format!("{}__Future Plc/Yachting Monthly", BASE_PATH),
	);
	destination_map.insert(
		"Yachting World ".to_string(),
		format!("{}__Future Plc/Yachting World", BASE_PATH),
	);
	destination_map.insert(
		"Future's ".to_string(),
		format!("{}__Future Plc/_MagBooks", BASE_PATH),
	);

	// Golden Peak Media
	destination_map.insert(
		"Artists Magazine ".to_string(),
		format!("{}__Golden Peak Media/Artists Magazine", BASE_PATH),
	);
	destination_map.insert(
		"Pastel Journal ".to_string(),
		format!("{}__Golden Peak Media/Pastel Journal", BASE_PATH),
	);
	destination_map.insert(
		"Southwest Art ".to_string(),
		format!("{}__Golden Peak Media/Southwest Art", BASE_PATH),
	);
	destination_map.insert(
		"Watercolor Artist ".to_string(),
		format!("{}__Golden Peak Media/Watercolor Artist", BASE_PATH),
	);

	// Guardian News & Media
	destination_map.insert(
		"The Guardian Feast ".to_string(),
		format!("{}__Guardian News & Media/The Guardian Feast", BASE_PATH),
	);
	destination_map.insert(
		"The Guardian Weekly ".to_string(),
		format!("{}__Guardian News & Media/The Guardian Weekly", BASE_PATH),
	);
	destination_map.insert(
		"The Observer Food Monthly ".to_string(),
		format!("{}__Guardian News & Media/The Observer Food Monthly", BASE_PATH),
	);
	destination_map.insert(
		"The Observer Magazine ".to_string(),
		format!("{}__Guardian News & Media/The Observer Magazine", BASE_PATH),
	);

	// Guideline Publications
	destination_map.insert(
		"Fantasy Figures ".to_string(),
		format!("{}__Guideline Publications/Fantasy Figures", BASE_PATH),
	);
	destination_map.insert(
		"Military Modelcraft International ".to_string(),
		format!("{}__Guideline Publications/Military Modelcraft International", BASE_PATH),
	);
	destination_map.insert(
		"Scale Aircraft Modelling ".to_string(),
		format!("{}__Guideline Publications/Scale Aircraft Modelling", BASE_PATH),
	);
	destination_map.insert(
		"Toy Soldier Collector ".to_string(),
		format!("{}__Guideline Publications/Toy Soldier Collector", BASE_PATH),
	);

	// Gun Digest Media
	destination_map.insert(
		"Blade 20".to_string(),
		format!("{}__Gun Digest Media/Blade", BASE_PATH),
	);

	// H Bauer
	destination_map.insert(
		"Angling Times ".to_string(),
		format!("{}__H Bauer/Angling Times", BASE_PATH),
	);
	destination_map.insert(
		"Bird Watching UK ".to_string(),
		format!("{}__H Bauer/Bird Watching UK", BASE_PATH),
	);
	destination_map.insert(
		"Country Walking ".to_string(),
		format!("{}__H Bauer/Country Walking", BASE_PATH),
	);
	destination_map.insert(
		"Empire ".to_string(),
		format!("{}__H Bauer/Empire", BASE_PATH),
	);
	destination_map.insert(
		"Garden Answers ".to_string(),
		format!("{}__H Bauer/Garden Answers", BASE_PATH),
	);
	destination_map.insert(
		"Garden News ".to_string(),
		format!("{}__H Bauer/Garden News", BASE_PATH),
	);
	destination_map.insert(
		"Landscape UK ".to_string(),
		format!("{}__H Bauer/Landscape UK", BASE_PATH),
	);
	destination_map.insert(
		"Modern Gardens ".to_string(),
		format!("{}__H Bauer/Modern Gardens", BASE_PATH),
	);
	destination_map.insert(
		"Mojo ".to_string(),
		format!("{}__H Bauer/Mojo", BASE_PATH),
	);
	destination_map.insert(
		"Trail UK ".to_string(),
		format!("{}__H Bauer/Trail UK", BASE_PATH),
	);

	// Habari Media
	destination_map.insert(
		"Food & Home Entertaining ".to_string(),
		format!("{}__Habari Media/Food & Home Entertaining", BASE_PATH),
	);

	// Hearst
	destination_map.insert(
		"Country Living ".to_string(),
		format!("{}__Hearst/Country Living", BASE_PATH),
	);
	destination_map.insert(
		"Delish ".to_string(),
		format!("{}__Hearst/Delish", BASE_PATH),
	);
	destination_map.insert(
		"Elle Decor ".to_string(),
		format!("{}__Hearst/Elle Decor", BASE_PATH),
	);
	destination_map.insert(
		"Elle Decor IT ".to_string(),
		format!("{}__Hearst/Elle Decor IT", BASE_PATH),
	);
	destination_map.insert(
		"Elle Decoration UK ".to_string(),
		format!("{}__Hearst/Elle Decoration UK", BASE_PATH),
	);
	destination_map.insert(
		"Esquire ".to_string(),
		format!("{}__Hearst/Esquire", BASE_PATH),
	);
	destination_map.insert(
		"Esquire UK ".to_string(),
		format!("{}__Hearst/Esquire UK", BASE_PATH),
	);
	destination_map.insert(
		"Food Network Magaizne ".to_string(),
		format!("{}__Hearst/Food Network Magaizne", BASE_PATH),
	);
	destination_map.insert(
		"Good Housekeeping UK ".to_string(),
		format!("{}__Hearst/Good Housekeeping UK", BASE_PATH),
	);
	destination_map.insert(
		"Good Housekeeping US ".to_string(),
		format!("{}__Hearst/Good Housekeeping US", BASE_PATH),
	);
	destination_map.insert(
		"House Beautiful UK ".to_string(),
		format!("{}__Hearst/House Beautiful UK", BASE_PATH),
	);
	destination_map.insert(
		"House Beautiful US ".to_string(),
		format!("{}__Hearst/House Beautiful US", BASE_PATH),
	);
	destination_map.insert(
		"Men's Health 20".to_string(),
		format!("{}__Hearst/Men's Health", BASE_PATH),
	);
	destination_map.insert(
		"Popular Mechanics ".to_string(),
		format!("{}__Hearst/Popular Mechanics", BASE_PATH),
	);
	destination_map.insert(
		"Runner's World ".to_string(),
		format!("{}__Hearst/Runner's World", BASE_PATH),
	);
	destination_map.insert(
		"Town & Country ".to_string(),
		format!("{}__Hearst/Town & Country", BASE_PATH),
	);
	destination_map.insert(
		"Veranda ".to_string(),
		format!("{}__Hearst/Veranda", BASE_PATH),
	);
	destination_map.insert(
		"Woman's Day ".to_string(),
		format!("{}__Hearst/Woman's Day", BASE_PATH),
	);

	// HistoryNet
	destination_map.insert(
		"America's Civil War ".to_string(),
		format!("{}__HistoryNet/America's Civil War", BASE_PATH),
	);
	destination_map.insert(
		"American History ".to_string(),
		format!("{}__HistoryNet/American History", BASE_PATH),
	);
	destination_map.insert(
		"Aviation History ".to_string(),
		format!("{}__HistoryNet/Aviation History", BASE_PATH),
	);
	destination_map.insert(
		"Vietnam ".to_string(),
		format!("{}__HistoryNet/Vietnam", BASE_PATH),
	);
	destination_map.insert(
		"Wild West ".to_string(),
		format!("{}__HistoryNet/Wild West", BASE_PATH),
	);
	destination_map.insert(
		"World War II ".to_string(),
		format!("{}__HistoryNet/World War II", BASE_PATH),
	);

	// Hoffman Media
	destination_map.insert(
		"Bake from Scratch ".to_string(),
		format!("{}__Hoffman Media/Bake from Scratch", BASE_PATH),
	);
	destination_map.insert(
		"Cooking with Paula Deen ".to_string(),
		format!("{}__Hoffman Media/Cooking with Paula Deen", BASE_PATH),
	);
	destination_map.insert(
		"Louisiana Cookin".to_string(),
		format!("{}__Hoffman Media/Louisiana Cookin", BASE_PATH),
	);
	destination_map.insert(
		"Southern Cast Iron ".to_string(),
		format!("{}__Hoffman Media/Southern Cast Iron", BASE_PATH),
	);
	destination_map.insert(
		"Southern Home ".to_string(),
		format!("{}__Hoffman Media/Southern Home", BASE_PATH),
	);
	destination_map.insert(
		"Southern Lady ".to_string(),
		format!("{}__Hoffman Media/Southern Lady", BASE_PATH),
	);
	destination_map.insert(
		"Taste of the South ".to_string(),
		format!("{}__Hoffman Media/Taste of the South", BASE_PATH),
	);
	destination_map.insert(
		"TeaTime ".to_string(),
		format!("{}__Hoffman Media/TeaTime", BASE_PATH),
	);
	destination_map.insert(
		"The Cottage Journal ".to_string(),
		format!("{}__Hoffman Media/The Cottage Journal", BASE_PATH),
	);
	destination_map.insert(
		"Victoria ".to_string(),
		format!("{}__Hoffman Media/Victoria", BASE_PATH),
	);

	// IDG
	destination_map.insert(
		"Android Advisor ".to_string(),
		format!("{}__IDG/Android Advisor", BASE_PATH),
	);
	destination_map.insert(
		"iPad & iPhone User ".to_string(),
		format!("{}__IDG/iPad & iPhone User", BASE_PATH),
	);
	destination_map.insert(
		"Macworld 20".to_string(),
		format!("{}__IDG/Macworld", BASE_PATH),
	);
	destination_map.insert(
		"Macworld UK ".to_string(),
		format!("{}__IDG/Macworld UK", BASE_PATH),
	);
	destination_map.insert(
		"PCWorld ".to_string(),
		format!("{}__IDG/PCWorld", BASE_PATH),
	);
	destination_map.insert(
		"Tech Advisor ".to_string(),
		format!("{}__IDG/Tech Advisor", BASE_PATH),
	);

	// IEEE
	destination_map.insert(
		"IEEE ".to_string(),
		format!("{}__IEEE", BASE_PATH),	// TODO: 세분화 필요.
	);

	// Immediate
	destination_map.insert(
		"BBC Easy Cook ".to_string(),
		format!("{}__Immediate/BBC Easy Cook", BASE_PATH),
	);
	destination_map.insert(
		"BBC Gardeners' World ".to_string(),
		format!("{}__Immediate/BBC Gardeners' World", BASE_PATH),
	);
	destination_map.insert(
		"BBC Good Food 20".to_string(),
		format!("{}__Immediate/BBC Good Food", BASE_PATH),
	);
	destination_map.insert(
		"BBC Good Food ME ".to_string(),
		format!("{}__Immediate/BBC Good Food ME", BASE_PATH),
	);
	destination_map.insert(
		"BBC History ".to_string(),
		format!("{}__Immediate/BBC History", BASE_PATH),
	);
	destination_map.insert(
		"BBC Music ".to_string(),
		format!("{}__Immediate/BBC Music Magazine", BASE_PATH),
	);
	destination_map.insert(
		"BBC Science Focus ".to_string(),
		format!("{}__Immediate/BBC Science Focus", BASE_PATH),
	);
	destination_map.insert(
		"BBC Wildlife ".to_string(),
		format!("{}__Immediate/BBC Wildlife", BASE_PATH),
	);
	destination_map.insert(
		"Gardens Illustrated ".to_string(),
		format!("{}__Immediate/Gardens Illustrated", BASE_PATH),
	);
	destination_map.insert(
		"Home Style UK ".to_string(),
		format!("{}__Immediate/Home Style UK", BASE_PATH),
	);
	destination_map.insert(
		"Homes & Antiques ".to_string(),
		format!("{}__Immediate/Homes & Antiques", BASE_PATH),
	);
	destination_map.insert(
		"Olive ".to_string(),
		format!("{}__Immediate/Olive", BASE_PATH),
	);
	destination_map.insert(
		"Radio Times ".to_string(),
		format!("{}__Immediate/Radio Times", BASE_PATH),
	);
	destination_map.insert(
		"Who Do You Think You Are ".to_string(),
		format!("{}__Immediate/Who Do You Think You Are", BASE_PATH),
	);
	destination_map.insert(
		"Your Home UK ".to_string(),
		format!("{}__Immediate/Your Home UK", BASE_PATH),
	);

	// International Artist Publishing
	destination_map.insert(
		"American Art Collector ".to_string(),
		format!("{}__International Artist Publishing/American Art Collector", BASE_PATH),
	);
	destination_map.insert(
		"American Fine Art ".to_string(),
		format!("{}__International Artist Publishing/American Fine Art", BASE_PATH),
	);
	destination_map.insert(
		"International Artist ".to_string(),
		format!("{}__International Artist Publishing/International Artist", BASE_PATH),
	);
	destination_map.insert(
		"Native American Art ".to_string(),
		format!("{}__International Artist Publishing/Native American Art", BASE_PATH),
	);
	destination_map.insert(
		"Western Art Collector ".to_string(),
		format!("{}__International Artist Publishing/Western Art Collector", BASE_PATH),
	);

	// KCK Media
	destination_map.insert(
		"audioXpress ".to_string(),
		format!("{}__KCK Media/audioXpress", BASE_PATH),
	);
	destination_map.insert(
		"Circuit Cellar ".to_string(),
		format!("{}__KCK Media/Circuit Cellar", BASE_PATH),
	);

	// Kalmbach Media
	destination_map.insert(
		"Astronomy 20".to_string(),
		format!("{}__Kalmbach Media/Astronomy", BASE_PATH),
	);
	destination_map.insert(
		"Discover 20".to_string(),
		format!("{}__Kalmbach Media/Discover", BASE_PATH),
	);
	destination_map.insert(
		"FineScale Modeler ".to_string(),
		format!("{}__Kalmbach Media/FineScale Modeler", BASE_PATH),
	);

	// Karwansaray Publishers
	destination_map.insert(
		"Ancient History ".to_string(),
		format!("{}__Karwansaray Publishers/Ancient History", BASE_PATH),
	);
	destination_map.insert(
		"Ancient Warfare ".to_string(),
		format!("{}__Karwansaray Publishers/Ancient Warfare", BASE_PATH),
	);
	destination_map.insert(
		"Medieval World ".to_string(),
		format!("{}__Karwansaray Publishers/Medieval World", BASE_PATH),
	);
	destination_map.insert(
		"Wargames Soldiers & Strategy Magazine ".to_string(),
		format!("{}__Karwansaray Publishers/Wargames Soldiers & Strategy Magazine", BASE_PATH),
	);

	// Kelsey
	destination_map.insert(
		"220 Triathlon ".to_string(),
		format!("{}__Kelsey/220 Triathlon", BASE_PATH),
	);
	destination_map.insert(
		"Amateur Gardening ".to_string(),
		format!("{}__Kelsey/Amateur Gardening", BASE_PATH),
	);
	destination_map.insert(
		"Amateur Photographer ".to_string(),
		format!("{}__Kelsey/Amateur Photographer", BASE_PATH),
	);
	destination_map.insert(
		"Boxing News ".to_string(),
		format!("{}__Kelsey/Boxing News", BASE_PATH),
	);
	destination_map.insert(
		"ClayCraft ".to_string(),
		format!("{}__Kelsey/ClayCraft", BASE_PATH),
	);
	destination_map.insert(
		"Coast 20".to_string(),
		format!("{}__Kelsey/Coast", BASE_PATH),
	);
	destination_map.insert(
		"Farm Machinery ".to_string(),
		format!("{}__Kelsey/Farm Machinery", BASE_PATH),
	);
	destination_map.insert(
		"Men's Fitness ".to_string(),
		format!("{}__Kelsey/Men's Fitness", BASE_PATH),
	);
	destination_map.insert(
		"Old Glory ".to_string(),
		format!("{}__Kelsey/Old Glory", BASE_PATH),
	);
	destination_map.insert(
		"Psychologies ".to_string(),
		format!("{}__Kelsey/Psychologies", BASE_PATH),
	);
	destination_map.insert(
		"Ships Monthly ".to_string(),
		format!("{}__Kelsey/Ships Monthly", BASE_PATH),
	);
	destination_map.insert(
		"Stuff ".to_string(),
		format!("{}__Kelsey/Stuff", BASE_PATH),
	);
	destination_map.insert(
		"The Country Smallholder ".to_string(),
		format!("{}__Kelsey/The Country Smallholder", BASE_PATH),
	);
	destination_map.insert(
		"The Great Outdoors ".to_string(),
		format!("{}__Kelsey/The Great Outdoors", BASE_PATH),
	);
	destination_map.insert(
		"Uncut ".to_string(),
		format!("{}__Kelsey/Uncut", BASE_PATH),
	);
	destination_map.insert(
		"World Soccer ".to_string(),
		format!("{}__Kelsey/World Soccer", BASE_PATH),
	);
	destination_map.insert(
		"World of Interiors ".to_string(),
		format!("{}__Kelsey/World of Interiors", BASE_PATH),
	);
	destination_map.insert(
		"World of Ships ".to_string(),
		format!("{}__Kelsey/World of Ships", BASE_PATH),
	);

	// Key
	destination_map.insert(
		"Aeroplane #".to_string(),
		format!("{}__Key/Aeroplane", BASE_PATH),
	);
	destination_map.insert(
		"Air International ".to_string(),
		format!("{}__Key/Air International", BASE_PATH),
	);
	destination_map.insert(
		"AirForces Monthly ".to_string(),
		format!("{}__Key/AirForces Monthly", BASE_PATH),
	);
	destination_map.insert(
		"Airfix Model World ".to_string(),
		format!("{}__Key/Airfix Model World", BASE_PATH),
	);
	destination_map.insert(
		"Aviation Archive ".to_string(),
		format!("{}__Key/Aviation Archive", BASE_PATH),
	);
	destination_map.insert(
		"Aviation News ".to_string(),
		format!("{}__Key/Aviation News", BASE_PATH),
	);
	destination_map.insert(
		"Britain at War ".to_string(),
		format!("{}__Key/Britain at War", BASE_PATH),
	);
	destination_map.insert(
		"Classic Military Vehicle ".to_string(),
		format!("{}__Key/Classic Military Vehicle", BASE_PATH),
	);
	destination_map.insert(
		"Combat Aircraft ".to_string(),
		format!("{}__Key/Combat Aircraft", BASE_PATH),
	);
	destination_map.insert(
		"FlyPast ".to_string(),
		format!("{}__Key/FlyPast", BASE_PATH),
	);

	// Mark Allen Group
	destination_map.insert(
		"Aerospace Testing International ".to_string(),
		format!("{}__Mark Allen Group/Aerospace Testing International", BASE_PATH),
	);
	destination_map.insert(
		"Gramophone ".to_string(),
		format!("{}__Mark Allen Group/Gramophone", BASE_PATH),
	);
	destination_map.insert(
		"Jazzwise ".to_string(),
		format!("{}__Mark Allen Group/Jazzwise", BASE_PATH),
	);
	destination_map.insert(
		"New Electronics ".to_string(),
		format!("{}__Mark Allen Group/New Electronics", BASE_PATH),
	);
	destination_map.insert(
		"Songlines ".to_string(),
		format!("{}__Mark Allen Group/Songlines", BASE_PATH),
	);
	destination_map.insert(
		"The Engineer ".to_string(),
		format!("{}__Mark Allen Group/The Engineer", BASE_PATH),
	);

	// Media Group PTE
	destination_map.insert(
		"Epicure SG ".to_string(),
		format!("{}__Media Group PTE/Epicure SG", BASE_PATH),
	);
	destination_map.insert(
		"SquareRooms ".to_string(),
		format!("{}__Media Group PTE/SquareRooms", BASE_PATH),
	);

	// Meredith
	destination_map.insert(
		"Allrecipes ".to_string(),
		format!("{}__Meredith/Allrecipes", BASE_PATH),
	);
	destination_map.insert(
		"Better Homes & Gardens ".to_string(),
		format!("{}__Meredith/Better Homes & Gardens", BASE_PATH),
	);
	destination_map.insert(
		"Coastal Living ".to_string(),
		format!("{}__Meredith/Coastal Living", BASE_PATH),
	);
	destination_map.insert(
		"Cooking Light ".to_string(),
		format!("{}__Meredith/Cooking Light", BASE_PATH),
	);
	destination_map.insert(
		"EatingWell ".to_string(),
		format!("{}__Meredith/EatingWell", BASE_PATH),
	);
	destination_map.insert(
		"Food & Wine ".to_string(),
		format!("{}__Meredith/Food & Wine", BASE_PATH),
	);
	destination_map.insert(
		"Modern Farmhouse Style ".to_string(),
		format!("{}__Meredith/Modern Farmhouse Style", BASE_PATH),
	);
	destination_map.insert(
		"Real Simple ".to_string(),
		format!("{}__Meredith/Real Simple", BASE_PATH),
	);
	destination_map.insert(
		"Successful Farming ".to_string(),
		format!("{}__Meredith/Successful Farming", BASE_PATH),
	);

	// Mindfield Digital
	destination_map.insert(
		"AppleMagazine ".to_string(),
		format!("{}__Mindfield Digital/AppleMagazine", BASE_PATH),
	);
	destination_map.insert(
		"TechLife News ".to_string(),
		format!("{}__Mindfield Digital/TechLife News", BASE_PATH),
	);

	// Morton
	destination_map.insert(
		"Best of British ".to_string(),
		format!("{}__Morton/Best of British", BASE_PATH),
	);
	destination_map.insert(
		"Kitchen Garden ".to_string(),
		format!("{}__Morton/Kitchen Garden", BASE_PATH),
	);
	destination_map.insert(
		"Model Boats ".to_string(),
		format!("{}__Morton/Model Boats", BASE_PATH),
	);

	// NetMag Media
	destination_map.insert(
		"Architects Datafile ".to_string(),
		format!("{}__NetMag Media/Architects Datafile", BASE_PATH),
	);
	destination_map.insert(
		"Housebuilder & Developer ".to_string(),
		format!("{}__NetMag Media/Housebuilder & Developer", BASE_PATH),
	);
	destination_map.insert(
		"Selfbuilder + Homemaker ".to_string(),
		format!("{}__NetMag Media/Selfbuilder + Homemaker", BASE_PATH),
	);

	// News Life Media
	destination_map.insert(
		"Delicious AU ".to_string(),
		format!("{}__News Life Media/Delicious AU", BASE_PATH),
	);
	destination_map.insert(
		"Taste.com.au ".to_string(),
		format!("{}__News Life Media/Taste.com.au", BASE_PATH),
	);
	destination_map.insert(
		"Vogue Living AU ".to_string(),
		format!("{}__News Life Media/Vogue Living AU", BASE_PATH),
	);

	// Nextmedia
	destination_map.insert(
		"Frankie ".to_string(),
		format!("{}__Nextmedia/Frankie", BASE_PATH),
	);
	destination_map.insert(
		"Gardening Australia ".to_string(),
		format!("{}__Nextmedia/Gardening Australia", BASE_PATH),
	);
	destination_map.insert(
		"Healthy Food Guide AU ".to_string(),
		format!("{}__Nextmedia/Healthy Food Guide AU", BASE_PATH),
	);
	destination_map.insert(
		"Science Illustrated AU ".to_string(),
		format!("{}__Nextmedia/Science Illustrated AU", BASE_PATH),
	);

	// Pam Communications
	destination_map.insert(
		"Electronic Sound ".to_string(),
		format!("{}__Pam Communications/Electronic Sound", BASE_PATH),
	);

	// Penske Media Corp.
	destination_map.insert(
		"Art in America ".to_string(),
		format!("{}__Penske Media Corp./Art in America", BASE_PATH),
	);
	destination_map.insert(
		"Artforum ".to_string(),
		format!("{}__Penske Media Corp./Artforum", BASE_PATH),
	);
	destination_map.insert(
		"Billboard ".to_string(),
		format!("{}__Penske Media Corp./Billboard", BASE_PATH),
	);
	destination_map.insert(
		"Rolling Stone 20".to_string(),
		format!("{}__Penske Media Corp./Rolling Stone", BASE_PATH),
	);
	destination_map.insert(
		"Rolling Stone ANZ ".to_string(),
		format!("{}__Penske Media Corp./Rolling Stone ANZ", BASE_PATH),
	);
	destination_map.insert(
		"Rolling Stone UK ".to_string(),
		format!("{}__Penske Media Corp./Rolling Stone UK", BASE_PATH),
	);
	destination_map.insert(
		"The Hollywood Reporter ".to_string(),
		format!("{}__Penske Media Corp./The Hollywood Reporter", BASE_PATH),
	);

	// Prime Creative Media
	destination_map.insert(
		"BeanScene ".to_string(),
		format!("{}__Prime Creative Media/BeanScene", BASE_PATH),
	);

	// Project M Group
	destination_map.insert(
		"Goldmine ".to_string(),
		format!("{}__Project M Group/Goldmine", BASE_PATH),
	);
	destination_map.insert(
		"Revolver ".to_string(),
		format!("{}__Project M Group/Revolver", BASE_PATH),
	);

	// Raspberry Pi Press
	destination_map.insert(
		"Hello World ".to_string(),
		format!("{}__Raspberry Pi Press/Hello World", BASE_PATH),
	);
	destination_map.insert(
		"The MagPi ".to_string(),
		format!("{}__Raspberry Pi Press/The MagPi", BASE_PATH),
	);

	// Recurrent Ventures
	destination_map.insert(
		"Dwell ".to_string(),
		format!("{}__Recurrent Ventures/Dwell", BASE_PATH),
	);
	destination_map.insert(
		"Outdoor Life ".to_string(),
		format!("{}__Recurrent Ventures/Outdoor Life", BASE_PATH),
	);

	// SPH Media
	destination_map.insert(
		"HWM SG ".to_string(),
		format!("{}__SPH Media/HWM SG", BASE_PATH),
	);
	destination_map.insert(
		"Home & Decor SG ".to_string(),
		format!("{}__SPH Media/Home & Decor SG", BASE_PATH),
	);

	// Scan Magazine
	destination_map.insert(
		"Discover CleanTech ".to_string(),
		format!("{}__Scan Magazine/Discover CleanTech", BASE_PATH),
	);
	destination_map.insert(
		"Discover Germany ".to_string(),
		format!("{}__Scan Magazine/Discover Germany", BASE_PATH),
	);

	// Sigma Xi
	destination_map.insert(
		"American Scientist ".to_string(),
		format!("{}__Sigma Xi/American Scientist", BASE_PATH),
	);

	// Silverback Publishing
	destination_map.insert(
		"Blitzed ".to_string(),
		format!("{}__Silverback Publishing/Blitzed", BASE_PATH),
	);
	destination_map.insert(
		"Blocks #".to_string(),
		format!("{}__Silverback Publishing/Blocks", BASE_PATH),
	);
	destination_map.insert(
		"Shindig ".to_string(),
		format!("{}__Silverback Publishing/Shindig", BASE_PATH),
	);

	// Sola Group
	destination_map.insert(
		"Kitchen & Bath Design News ".to_string(),
		format!("{}__Sola Group/Kitchen & Bath Design News", BASE_PATH),
	);
	destination_map.insert(
		"Residential Design ".to_string(),
		format!("{}__Sola Group/Residential Design", BASE_PATH),
	);

	// Stella Novus
	destination_map.insert(
		"Ancient Origins Magazine ".to_string(),
		format!("{}__Stella Novus/Ancient Origins Magazine", BASE_PATH),
	);

	// Sunray Publications
	destination_map.insert(
		"Artist's Drawing & Inspiration ".to_string(),
		format!("{}__Sunray Publications/Artist's Drawing & Inspiration", BASE_PATH),
	);
	destination_map.insert(
		"Artist's Palette ".to_string(),
		format!("{}__Sunray Publications/Artist's Palette", BASE_PATH),
	);
	destination_map.insert(
		"Australian How to Paint ".to_string(),
		format!("{}__Sunray Publications/Australian How to Paint", BASE_PATH),
	);
	
	// TEN Publishing
	destination_map.insert(
		"Recoil ".to_string(),
		format!("{}__TEN Publishing/Recoil", BASE_PATH),
	);

	// The Chelsea Magazine Company
	destination_map.insert(
		"Artists & Illustrators ".to_string(),
		format!("{}__The Chelsea Magazine Company/Artists & Illustrators", BASE_PATH),
	);
	destination_map.insert(
		"Britain 20".to_string(),
		format!("{}__The Chelsea Magazine Company/Britain", BASE_PATH),
	);
	destination_map.insert(
		"Classic Boat ".to_string(),
		format!("{}__The Chelsea Magazine Company/Classic Boat", BASE_PATH),
	);
	destination_map.insert(
		"Discover Britain ".to_string(),
		format!("{}__The Chelsea Magazine Company/Discover Britain", BASE_PATH),
	);
	destination_map.insert(
		"Sailing Today ".to_string(),
		format!("{}__The Chelsea Magazine Company/Sailing Today", BASE_PATH),
	);
	destination_map.insert(
		"The English Garden ".to_string(),
		format!("{}__The Chelsea Magazine Company/The English Garden", BASE_PATH),
	);
	destination_map.insert(
		"The English Home ".to_string(),
		format!("{}__The Chelsea Magazine Company/The English Home", BASE_PATH),
	);
	destination_map.insert(
		"The London Magazine ".to_string(),
		format!("{}__The Chelsea Magazine Company/The London Magazine", BASE_PATH),
	);

	// The Economist Newspaper
	destination_map.insert(
		"Economist Intelligence Unit ".to_string(),
		format!("{}__The Economist Newspaper/Economist Intelligence Unit", BASE_PATH),
	);

	// The Nation Company
	destination_map.insert(
		"The Nation ".to_string(),
		format!("{}__The Nation Company/The Nation", BASE_PATH),
	);

	// The New York Times Company
	destination_map.insert(
		"The New York Times Arts & Leisure ".to_string(),
		format!("{}__The New York Times Company/The New York Times Arts & Leisure", BASE_PATH),
	);
	destination_map.insert(
		"The New York Times Book Review ".to_string(),
		format!("{}__The New York Times Company/The New York Times Book Review", BASE_PATH),
	);
	destination_map.insert(
		"The New York Times Magazine ".to_string(),
		format!("{}__The New York Times Company/The New York Times Magazine", BASE_PATH),
	);
	destination_map.insert(
		"The New York Times Style Magazine ".to_string(),
		format!("{}__The New York Times Company/The New York Times Style Magazine", BASE_PATH),
	);

	// Times Media
	destination_map.insert(
		"The Sunday Times Culture ".to_string(),
		format!("{}__Times Media/The Sunday Times Culture", BASE_PATH),
	);
	destination_map.insert(
		"The Sunday Times Magazine ".to_string(),
		format!("{}__Times Media/The Sunday Times Magazine", BASE_PATH),
	);
	destination_map.insert(
		"The Sunday Times Style ".to_string(),
		format!("{}__Times Media/The Sunday Times Style", BASE_PATH),
	);
	destination_map.insert(
		"The Times Magazine ".to_string(),
		format!("{}__Times Media/The Times Magazine", BASE_PATH),
	);

	// Trusted Media Brands
	destination_map.insert(
		"Birds & Blooms ".to_string(),
		format!("{}__Trusted Media Brands/Birds & Blooms", BASE_PATH),
	);
	destination_map.insert(
		"Taste of Home ".to_string(),
		format!("{}__Trusted Media Brands/Taste of Home", BASE_PATH),
	);
	destination_map.insert(
		"The Family Handyman ".to_string(),
		format!("{}__Trusted Media Brands/The Family Handyman", BASE_PATH),
	);

	// TVA
	destination_map.insert(
		"Canadian Living ".to_string(),
		format!("{}__TVA/Canadian Living", BASE_PATH),
	);

	// Universal Media Co.
	destination_map.insert(
		"Australian Country ".to_string(),
		format!("{}__Universal Media Co./Australian Country", BASE_PATH),
	);

	// Wanderlust Travel Media
	destination_map.insert(
		"Wanderlust ".to_string(),
		format!("{}__Wanderlust Travel Media/Wanderlust", BASE_PATH),
	);
	
    // Warners
	destination_map.insert(
		"Bake & Decorate ".to_string(),
		format!("{}__Warners/Bake & Decorate", BASE_PATH),
	);
	destination_map.insert(
		"Battleships of WWII ".to_string(),
		format!("{}__Warners/Battleships of WWII", BASE_PATH),
	);
	destination_map.insert(
		"Birdwatch UK ".to_string(),
		format!("{}__Warners/Birdwatch UK", BASE_PATH),
	);
	destination_map.insert(
		"Coin Collector ".to_string(),
		format!("{}__Warners/Coin Collector", BASE_PATH),
	);
	destination_map.insert(
		"Collectors Gazette ".to_string(),
		format!("{}__Warners/Collectors Gazette", BASE_PATH),
	);
	destination_map.insert(
		"Diecast Collector ".to_string(),
		format!("{}__Warners/Diecast Collector", BASE_PATH),
	);
	destination_map.insert(
		"Dolls House & Miniature Scene ".to_string(),
		format!("{}__Warners/Dolls House & Miniature Scene", BASE_PATH),
	);
	destination_map.insert(
		"Family Tree UK ".to_string(),
		format!("{}__Warners/Family Tree UK", BASE_PATH),
	);
	destination_map.insert(
		"History Scotland ".to_string(),
		format!("{}__Warners/History Scotland", BASE_PATH),
	);
	destination_map.insert(
		"Iron Cross ".to_string(),
		format!("{}__Warners/Iron Cross", BASE_PATH),
	);
	destination_map.insert(
		"Land Rover Monthly ".to_string(),
		format!("{}__Warners/Land Rover Monthly", BASE_PATH),
	);
	destination_map.insert(
		"Leisure Painter ".to_string(),
		format!("{}__Warners/Leisure Painter", BASE_PATH),
	);
	destination_map.insert(
		"Master Detective ".to_string(),
		format!("{}__Warners/Master Detective", BASE_PATH),
	);
	destination_map.insert(
		"Miniature Wargames ".to_string(),
		format!("{}__Warners/Miniature Wargames", BASE_PATH),
	);
	destination_map.insert(
		"Practical Fishkeeping ".to_string(),
		format!("{}__Warners/Practical Fishkeeping", BASE_PATH),
	);
	destination_map.insert(
		"Practical Wireless ".to_string(),
		format!("{}__Warners/Practical Wireless", BASE_PATH),
	);
	destination_map.insert(
		"Scottish Field ".to_string(),
		format!("{}__Warners/Scottish Field", BASE_PATH),
	);
	destination_map.insert(
		"Stamp Collector ".to_string(),
		format!("{}__Warners/Stamp Collector", BASE_PATH),
	);
	destination_map.insert(
		"Tabletop Gaming ".to_string(),
		format!("{}__Warners/Tabletop Gaming", BASE_PATH),
	);
	destination_map.insert(
		"The Armourer ".to_string(),
		format!("{}__Warners/The Armourer", BASE_PATH),
	);
	destination_map.insert(
		"The Artist ".to_string(),
		format!("{}__Warners/The Artist", BASE_PATH),
	);
	destination_map.insert(
		"The Searcher ".to_string(),
		format!("{}__Warners/The Searcher", BASE_PATH),
	);
	destination_map.insert(
		"Toy Collectors ".to_string(),
		format!("{}__Warners/Toy Collectors", BASE_PATH),
	);
	destination_map.insert(
		"Wargames Illustrated ".to_string(),
		format!("{}__Warners/Wargames Illustrated", BASE_PATH),
	);
	destination_map.insert(
		"Writing Magazine ".to_string(),
		format!("{}__Warners/Writing Magazine", BASE_PATH),
	);

	// WTWH Media
	destination_map.insert(
		"Design World ".to_string(),
		format!("{}__WTWH Media/Design World", BASE_PATH),
	);

/*
	destination_map.insert(
		"abcd ".to_string(),
		format!("{}__abc/abcd", BASE_PATH),
	);
*/

    // 더 많은 경로 추가 가능
    destination_map
}
