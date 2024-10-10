use std::collections::HashMap;

// 공통 경로를 상수로 선언
const BASE_PATH: &str = "./Downloads/_DONE/_Magazines/";

pub fn get_destination_map() -> HashMap<String, String> {
    let mut destination_map: HashMap<String, String> = HashMap::new();

    // format!을 사용하여 각 경로를 문자열로 생성
    // TODO: 이름이 겹치지 않도록 확인! & 잡지사 별로 묶을지 ABC순서대로 나열할지 고민 필요.

	// AAAS
	destination_map.insert(
		"Science 20".to_string(),
		format!("{}__AAAS/Science", BASE_PATH),
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

    // 더 많은 경로 추가 가능
    destination_map
}
