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

	// ABK Publications
	destination_map.insert(
		"Australian Birdkeeper ".to_string(),
		format!("{}__ABK Publications/Australian Birdkeeper", BASE_PATH),
	);

	// Abode Publishing
	destination_map.insert(
		"Abode2 ".to_string(),
		format!("{}__Abode Publishing/Abode2", BASE_PATH),
	);

	// ACC Art Books
	destination_map.insert(
		"Antique Collecting ".to_string(),
		format!("{}__ACC Art Books/Antique Collecting", BASE_PATH),
	);

	// Aceville Publications
	destination_map.insert(
		"What Franchise ".to_string(),
		format!("{}__Aceville Publications/What Franchise", BASE_PATH),
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
		"Fine Woodworking ".to_string(),
		format!("{}__Active Interest Media/Fine Woodworking", BASE_PATH),
	);
	destination_map.insert(
		"Garden Gate ".to_string(),
		format!("{}__Active Interest Media/Garden Gate", BASE_PATH),
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
		"Multihull Power & Sail ".to_string(),
		format!("{}__Active Interest Media/Multihull Power & Sail", BASE_PATH),
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
		"Power & Motoryacht ".to_string(),
		format!("{}__Active Interest Media/Power & Motoryacht", BASE_PATH),
	);
	destination_map.insert(
		"Sail - ".to_string(),
		format!("{}__Active Interest Media/Sail", BASE_PATH),
	);
	destination_map.insert(
		"Sail 20".to_string(),
		format!("{}__Active Interest Media/Sail", BASE_PATH),
	);
	destination_map.insert(
		"Soundings ".to_string(),
		format!("{}__Active Interest Media/Soundings", BASE_PATH),
	);
	destination_map.insert(
		"Traditional Building ".to_string(),
		format!("{}__Active Interest Media/Traditional Building", BASE_PATH),
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

	// Adirondack Explorer
	destination_map.insert(
		"Adirondack Explorer ".to_string(),
		format!("{}__Adirondack Explorer/Adirondack Explorer", BASE_PATH),
	);

	// Adirondack Life
	destination_map.insert(
		"Adirondack Life ".to_string(),
		format!("{}__Adirondack Life/Adirondack Life", BASE_PATH),
	);

	// Adventure Entertainment
	destination_map.insert(
		"Trail Run ANZ ".to_string(),
		format!("{}__Adventure Entertainment/Trail Run ANZ", BASE_PATH),
	);
	destination_map.insert(
		"Wild #".to_string(),
		format!("{}__Adventure Entertainment/Wild", BASE_PATH),
	);

	// AFAR
	destination_map.insert(
		"AFAR ".to_string(),
		format!("{}__AFAR/AFAR", BASE_PATH),
	);

	// After Dinner Conversation
	destination_map.insert(
		"After Dinner Conversation ".to_string(),
		format!("{}__After Dinner Conversation/After Dinner Conversation", BASE_PATH),
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

	// Agriconnect
	destination_map.insert(
		"Farmers Guardian ".to_string(),
		format!("{}__Agriconnect/Farmers Guardian", BASE_PATH),
	);

	// AIAA
	destination_map.insert(
		"Aerospace America ".to_string(),
		format!("{}__AIAA/Aerospace America", BASE_PATH),
	);

	// AIM Communications
	destination_map.insert(
		"American Cake Decorating ".to_string(),
		format!("{}__AIM Communications/American Cake Decorating", BASE_PATH),
	);

	// Air & Space Forces Association
	destination_map.insert(
		"Air & Space Forces ".to_string(),
		format!("{}__Air & Space Forces Association/Air & Space Forces", BASE_PATH),
	);

	// Air Age Media
	destination_map.insert(
		"Flight Journal ".to_string(),
		format!("{}__Air Age Media/Flight Journal", BASE_PATH),
	);
	destination_map.insert(
		"Model Airplane News ".to_string(),
		format!("{}__Air Age Media/Model Airplane News", BASE_PATH),
	);

	// AJ Bell Media
	destination_map.insert(
		"Shares ".to_string(),
		format!("{}__AJ Bell Media/Shares", BASE_PATH),
	);

	// Alpine Publishing Group
	destination_map.insert(
		"Denver Life ".to_string(),
		format!("{}__Alpine Publishing Group/Denver Life", BASE_PATH),
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

	// American Chess Magazine
	destination_map.insert(
		"American Chess Magazine ".to_string(),
		format!("{}__American Chess Magazine/American Chess Magazine", BASE_PATH),
	);

	// American Craft Council
	destination_map.insert(
		"American Craft ".to_string(),
		format!("{}__American Craft Council/American Craft", BASE_PATH),
	);

	// American Institute of Steel Construction
	destination_map.insert(
		"Modern Steel Construction ".to_string(),
		format!("{}__American Institute of Steel Construction/Modern Steel Construction", BASE_PATH),
	);

	// American Library Association
	destination_map.insert(
		"American Libraries ".to_string(),
		format!("{}__American Library Association/American Libraries", BASE_PATH),
	);
	destination_map.insert(
		"Booklist 20".to_string(),
		format!("{}__American Library Association/Booklist", BASE_PATH),
	);
	destination_map.insert(
		"Booklist Reader ".to_string(),
		format!("{}__American Library Association/Booklist Reader", BASE_PATH),
	);

	// American Society of Landscape Architects
	destination_map.insert(
		"Landscape Architecture Magazine ".to_string(),
		format!(
			"{}__American Society of Landscape Architects/Landscape Architecture Magazine",
			BASE_PATH
		),
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

	// Animation Magazine
	destination_map.insert(
		"Animation Magazine ".to_string(),
		format!("{}__Animation Magazine/Animation Magazine", BASE_PATH),
	);

	// Anja Publications
	destination_map.insert(
		"Philosophy Now ".to_string(),
		format!("{}__Anja Publications/Philosophy Now", BASE_PATH),
	);

	// Annex Business Media
	destination_map.insert(
		"Design Engineering ".to_string(),
		format!("{}__Annex Business Media/Design Engineering", BASE_PATH),
	);
	destination_map.insert(
		"Electronic Products & Technology ".to_string(),
		format!("{}__Annex Business Media/Electronic Products & Technology", BASE_PATH),
	);
	destination_map.insert(
		"Manufacturing Automation ".to_string(),
		format!("{}__Annex Business Media/Manufacturing Automation", BASE_PATH),
	);
	destination_map.insert(
		"Wings 20".to_string(),
		format!("{}__Annex Business Media/Wings", BASE_PATH),
	);

	// Annie's
	destination_map.insert(
		"Country Sampler ".to_string(),
		format!("{}__Annie's/Country Sampler", BASE_PATH),
	);

	// Anthem
	destination_map.insert(
		"Air Fryer Cookbook ".to_string(),
		format!("{}__Anthem/Air Fryer Cookbook", BASE_PATH),
	);
	destination_map.insert(
		"Anthem's ".to_string(),
		format!("{}__Anthem/_MagBooks", BASE_PATH),
	);
	destination_map.insert(
		"Classic Pop ".to_string(),
		format!("{}__Anthem/Classic Pop", BASE_PATH),
	);
	destination_map.insert(
		"Gluten-Free Cookbook ".to_string(),
		format!("{}__Anthem/Gluten-Free Cookbook", BASE_PATH),
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

	// AO-Media
	destination_map.insert(
		"Food & Beverage Reporter ".to_string(),
		format!("{}__AO-Media/Food & Beverage Reporter", BASE_PATH),
	);

	// Appeal Media & Events Group
	destination_map.insert(
		"Interior Appeal ".to_string(),
		format!("{}__Appeal Media & Events Group/Interior Appeal", BASE_PATH),
	);

	// Archaeological Institute of America
	destination_map.insert(
		"Archaeology 2".to_string(),
		format!("{}__Archaeological Institute of America/Archaeology", BASE_PATH),
	);

	// Archetech Media
	destination_map.insert(
		"Archetech ".to_string(),
		format!("{}__Archetech Media/Archetech", BASE_PATH),
	);

	// Architectural Woodwork Institute
	destination_map.insert(
		"Design Solutions ".to_string(),
		format!("{}__Architectural Woodwork Institute/Design Solutions", BASE_PATH),
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
		"Gourmet Traveller AU ".to_string(),
		format!("{}__Are Media/Gourmet Traveller AU", BASE_PATH),
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
	destination_map.insert(
		"Your Home and Garden NZ ".to_string(),
		format!("{}__Are Media/Your Home and Garden NZ", BASE_PATH),
	);

	// Army University Press
	destination_map.insert(
		"Military Review 20".to_string(),
		format!("{}__Army University Press/Military Review", BASE_PATH),
	);

	// Art & Antiques World Wide Media
	destination_map.insert(
		"Art & Antiques ".to_string(),
		format!("{}__Art & Antiques World Wide Media/Art & Antiques", BASE_PATH),
	);

	// Art Edited
	destination_map.insert(
		"Art Collector ".to_string(),
		format!("{}__Art Edited/Art Collector", BASE_PATH),
	);
	destination_map.insert(
		"Art Edit ".to_string(),
		format!("{}__Art Edited/Art Edit", BASE_PATH),
	);

	// Art Media
	destination_map.insert(
		"Art in America ".to_string(),
		format!("{}__Art Media/Art in America", BASE_PATH),
	);

	// Art New Zealand
	destination_map.insert(
		"Art New Zealand ".to_string(),
		format!("{}__Art New Zealand/Art New Zealand", BASE_PATH),
	);

	// ArtAsiaPacific Foundation
	destination_map.insert(
		"ArtAsiaPacific ".to_string(),
		format!("{}__ArtAsiaPacific Foundation/ArtAsiaPacific", BASE_PATH),
	);

	// ArtReview
	destination_map.insert(
		"ArtReview 20".to_string(),
		format!("{}__ArtReview/ArtReview", BASE_PATH),
	);
	destination_map.insert(
		"ArtReview Asia ".to_string(),
		format!("{}__ArtReview/ArtReview Asia", BASE_PATH),
	);

	// ArtTour International
	destination_map.insert(
		"ArtTour International ".to_string(),
		format!("{}__ArtTour International/ArtTour International", BASE_PATH),
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
		"Speciality Food ".to_string(),
		format!("{}__Artichoke/Speciality Food", BASE_PATH),
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
	destination_map.insert(
		"What Franchise ".to_string(),
		format!("{}__Artichoke/What Franchise", BASE_PATH),
	);

	// Artists Down Under
	destination_map.insert(
		"Artists Down Under ".to_string(),
		format!("{}__Artists Down Under/Artists Down Under", BASE_PATH),
	);

	// Ashdown
	destination_map.insert(
		"Bead & Jewellery ".to_string(),
		format!("{}__Ashdown/Bead & Jewellery", BASE_PATH),
	);
	destination_map.insert(
		"Dollhouse Miniatures ".to_string(),
		format!("{}__Ashdown/Dollhouse Miniatures", BASE_PATH),
	);
	destination_map.insert(
		"Dolls House World ".to_string(),
		format!("{}__Ashdown/Dolls House World", BASE_PATH),
	);

	// Asian Geographic Magazines
	destination_map.insert(
		"Asian Geographic ".to_string(),
		format!("{}__Asian Geographic Magazines/Asian Geographic", BASE_PATH),
	);

	// AspenCore
	destination_map.insert(
		"Electronic Products ".to_string(),
		format!("{}__AspenCore/Electronic Products", BASE_PATH),
	);

	// Associated Media Publishing
	destination_map.insert(
		"House and Leisure ".to_string(),
		format!("{}__Associated Media Publishing/House and Leisure", BASE_PATH),
	);

	// Astro Publishing
	destination_map.insert(
		"Free Astronomy ".to_string(),
		format!("{}__Astro Publishing/Free Astronomy", BASE_PATH),
	);

	// Association of Certified Fraud Examiners
	destination_map.insert(
		"Fraud Magazine ".to_string(),
		format!("{}__Association of Certified Fraud Examiners/Fraud Magazine", BASE_PATH),
	);

	// Athleisure Media
	destination_map.insert(
		"Athleisure ".to_string(),
		format!("{}__Athleisure Media/Athleisure", BASE_PATH),
	);

	// Athlon

	// AudioFile Publications
	destination_map.insert(
		"AudioFile ".to_string(),
		format!("{}__AudioFile Publications/AudioFile", BASE_PATH),
	);

	// Australian Geographic Holdings
	destination_map.insert(
		"Australian Geographic ".to_string(),
		format!("{}__Australian Geographic Holdings/Australian Geographic", BASE_PATH),
	);

	// Australian Traveller Media
	destination_map.insert(
		"Australian Traveller ".to_string(),
		format!("{}__Australian Traveller Media/Australian Traveller", BASE_PATH),
	);
	destination_map.insert(
		"International Traveller ".to_string(),
		format!("{}__Australian Traveller Media/International Traveller", BASE_PATH),
	);

	// Axel Springer SE
	destination_map.insert(
		"Politico EU ".to_string(),
		format!("{}__Axel Springer SE/Politico EU", BASE_PATH),
	);

	// AZURE Publishing
	destination_map.insert(
		"Azure 20".to_string(),
		format!("{}__AZURE Publishing/Azure", BASE_PATH),
	);
	destination_map.insert(
		"Designlines ".to_string(),
		format!("{}__AZURE Publishing/Designlines", BASE_PATH),
	);

	// Babcox
	destination_map.insert(
		"Engine Builder ".to_string(),
		format!("{}__Babcox/Engine Builder", BASE_PATH),
	);

	// Bandicoot Publishing
	destination_map.insert(
		"Art Almanac ".to_string(),
		format!("{}__Bandicoot Publishing/Art Almanac", BASE_PATH),
	);
	destination_map.insert(
		"Artist Profile ".to_string(),
		format!("{}__Bandicoot Publishing/Artist Profile", BASE_PATH),
	);

	// Bayshore History
	destination_map.insert(
		"The Civil War Monitor ".to_string(),
		format!("{}__Bayshore History/The Civil War Monitor", BASE_PATH),
	);

	// BCI Media
	destination_map.insert(
		"Architecture NZ ".to_string(),
		format!("{}__BCI Media/Architecture NZ", BASE_PATH),
	);

	// Biblical Archaelogy Review
	destination_map.insert(
		"Biblical Archaelogy Review ".to_string(),
		format!("{}__Biblical Archaelogy Review/Biblical Archaelogy Review", BASE_PATH),
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
		"FinTech Magazine ".to_string(),
		format!("{}__BizClik Media/FinTech Magazine", BASE_PATH),
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

	// Blues Matters
	destination_map.insert(
		"Blues Matters ".to_string(),
		format!("{}__Blues Matters/Blues Matters", BASE_PATH),
	);

	// BNP Media
	destination_map.insert(
		"Architectural Record ".to_string(),
		format!("{}__BNP Media/Architectural Record", BASE_PATH),
	);

	// Boat International Media
	destination_map.insert(
		"Boat International ".to_string(),
		format!("{}__Boat International Media/Boat International", BASE_PATH),
	);

	// Bonnier
	destination_map.insert(
		"Anti-Inflammatory ".to_string(),
		format!("{}__Bonnier/Anti-Inflammatory", BASE_PATH),
	);
	destination_map.insert(
		"Boating 20".to_string(),
		format!("{}__Bonnier/Boating", BASE_PATH),
	);
	destination_map.insert(
		"Boating - ".to_string(),
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
		"Marlin ".to_string(),
		format!("{}__Bonnier/Marlin", BASE_PATH),
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
		"Yachting 20".to_string(),
		format!("{}__Bonnier/Yachting", BASE_PATH),
	);

	// Bonsai Europ Publications
	destination_map.insert(
		"Bonsai Focus ".to_string(),
		format!("{}__Bonsai Europ Publications/Bonsai Focus", BASE_PATH),
	);

	// Bookpage and Promotion Inc
	destination_map.insert(
		"BookPage ".to_string(),
		format!("{}__Bookpage and Promotion Inc/BookPage", BASE_PATH),
	);

	// Brainspace Publishing
	destination_map.insert(
		"Brainspace ".to_string(),
		format!("{}__Brainspace Publishing/Brainspace", BASE_PATH),
	);

	// British Chess Magazine
	destination_map.insert(
		"British Chess Magazine ".to_string(),
		format!("{}__British Chess Magazine/British Chess Magazine", BASE_PATH),
	);

	// British Columbia Historical Federation
	destination_map.insert(
		"British Columbia History ".to_string(),
		format!(
			"{}__British Columbia Historical Federation/British Columbia History",
			BASE_PATH
		),
	);
	destination_map.insert(
		"British Columbia Magazine ".to_string(),
		format!(
			"{}__British Columbia Historical Federation/British Columbia Magazine",
			BASE_PATH
		),
	);

	// British Film Institute
	destination_map.insert(
		"Sight and Sound ".to_string(),
		format!("{}__British Film Institute/Sight and Sound", BASE_PATH),
	);

	// Canada Wide Media
	destination_map.insert(
		"Western Living ".to_string(),
		format!("{}__Canada Wide Media/Western Living", BASE_PATH),
	);

	// Canada's History Society
	destination_map.insert(
		"Canada's History ".to_string(),
		format!("{}__Canada's History Society/Canada's History", BASE_PATH),
	);

	// Canadian Geographical Society
	destination_map.insert(
		"Canadian Geographic ".to_string(),
		format!("{}__Canadian Geographical Society/Canadian Geographic", BASE_PATH),
	);

	// Canadian Home Publishers
	destination_map.insert(
		"House & Home 20".to_string(),
		format!("{}__Canadian Home Publishers/House & Home", BASE_PATH),
	);

	// Canadian Wildlife Federation
	destination_map.insert(
		"Canadian Wildlife ".to_string(),
		format!("{}__Canadian Wildlife Federation/Canadian Wildlife", BASE_PATH),
	);

	// Castle Media
	destination_map.insert(
		"Build It ".to_string(),
		format!("{}__Castle Media/Build It", BASE_PATH),
	);
	destination_map.insert(
		"SelfBuild & Design ".to_string(),
		format!("{}__Castle Media/SelfBuild & Design", BASE_PATH),
	);

	// Casual Game Revolution
	destination_map.insert(
		"Casual Game Insider ".to_string(),
		format!("{}__Casual Game Revolution/Casual Game Insider", BASE_PATH),
	);

	// Caxton Local Media
	destination_map.insert(
		"Farmer's Weekly ".to_string(),
		format!("{}__Caxton Local Media/Farmer's Weekly", BASE_PATH),
	);

	// Center for Inquiry
	destination_map.insert(
		"Skeptical Inquirer ".to_string(),
		format!("{}__Center for Inquiry/Skeptical Inquirer", BASE_PATH),
	);

	// CFE Media
	destination_map.insert(
		"Control Engineering ".to_string(),
		format!("{}__CFE Media/Control Engineering", BASE_PATH),
	);

	// Challenge Publications
	destination_map.insert(
		"Air Classics ".to_string(),
		format!("{}__Challenge Publications/Air Classics", BASE_PATH),
	);
	destination_map.insert(
		"Sea Classics ".to_string(),
		format!("{}__Challenge Publications/Sea Classics", BASE_PATH),
	);

	// Chedcorp Suppliers
	destination_map.insert(
		"Meal-Solve ".to_string(),
		format!("{}__Chedcorp Suppliers/Meal-Solve", BASE_PATH),
	);

	// Chef Publishing
	destination_map.insert(
		"Chef & Restaurant UK ".to_string(),
		format!("{}__Chef Publishing/Chef & Restaurant UK", BASE_PATH),
	);

	// China News Service
	destination_map.insert(
		"China Report ".to_string(),
		format!("{}__China News Service/China Report", BASE_PATH),
	);

	// CICG Americas
	destination_map.insert(
		"Beijing Review ".to_string(),
		format!("{}__CICG Americas/Beijing Review", BASE_PATH),
	);

	// Cinema Scope
	destination_map.insert(
		"Cinema Scope ".to_string(),
		format!("{}__Cinema Scope/Cinema Scope", BASE_PATH),
	);

	// Citrus Media
	destination_map.insert(
		"Market Magazine ".to_string(),
		format!("{}__Citrus Media/Market Magazine", BASE_PATH),
	);

	// Clarity Media Group
	destination_map.insert(
		"Washington Examiner ".to_string(),
		format!("{}__Clarity Media Group/Washington Examiner", BASE_PATH),
	);

	// CNB미디어 공간연구소
	destination_map.insert(
		"Space #".to_string(),
		format!("{}__CNB미디어 공간연구소/Space", BASE_PATH),
	);

	// Coastal Lifestyle
	destination_map.insert(
		"Coastal Lifestyle ".to_string(),
		format!("{}__Coastal Lifestyle/Coastal Lifestyle", BASE_PATH),
	);

	// Coastal Style Inc
	destination_map.insert(
		"Coastal Style 20".to_string(),
		format!("{}__Coastal Style Inc/Coastal Style", BASE_PATH),
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
		"GQ 20".to_string(),
		format!("{}__Condé Nast/GQ", BASE_PATH),
	);
	destination_map.insert(
		"GQ -".to_string(),
		format!("{}__Condé Nast/GQ", BASE_PATH),
	);
	destination_map.insert(
		"GQ UK ".to_string(),
		format!("{}__Condé Nast/GQ UK", BASE_PATH),
	);
	destination_map.insert(
		"House & Garden UK ".to_string(),
		format!("{}__Condé Nast/House & Garden UK", BASE_PATH),
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

	// Consumer Reports
	destination_map.insert(
		"Consumer Reports 20".to_string(),
		format!("{}__Consumer Reports/Consumer Reports", BASE_PATH),
	);
	destination_map.insert(
		"Consumer Reports - ".to_string(),
		format!("{}__Consumer Reports/Consumer Reports", BASE_PATH),
	);
	destination_map.insert(
		"Consumer Reports New Cars ".to_string(),
		format!("{}__Consumer Reports/Consumer Reports New Cars", BASE_PATH),
	);
	destination_map.insert(
		"Consumer Reports On Health ".to_string(),
		format!("{}__Consumer Reports/Consumer Reports On Health", BASE_PATH),
	);

	// Content Nation Media
	destination_map.insert(
		"House & Garden SA ".to_string(),
		format!("{}__Content Nation Media/House & Garden SA", BASE_PATH),
	);

	// Contista Media
	destination_map.insert(
		"British Travel Journal ".to_string(),
		format!("{}__Contista Media/British Travel Journal", BASE_PATH),
	);
	destination_map.insert(
		"Dream Escape Magazine ".to_string(),
		format!("{}__Contista Media/Dream Escape Magazine", BASE_PATH),
	);

	// Corporate Knights Inc
	destination_map.insert(
		"Corporate Knights ".to_string(),
		format!("{}__Corporate Knights Inc/Corporate Knights", BASE_PATH),
	);

	// Cottage Life Media
	destination_map.insert(
		"Cottage Life ".to_string(),
		format!("{}__Cottage Life Media/Cottage Life", BASE_PATH),
	);

	// Council for British Archaeology
	destination_map.insert(
		"British Archaeology ".to_string(),
		format!("{}__Council for British Archaeology/British Archaeology", BASE_PATH),
	);

	// Council on Foreign Relations
	destination_map.insert(
		"Foreign Affairs ".to_string(),
		format!("{}__Council on Foreign Relations/Foreign Affairs", BASE_PATH),
	);

	// Country & Town House
	destination_map.insert(
		"Country & Town House ".to_string(),
		format!("{}__Country & Town House/Country & Town House", BASE_PATH),
	);

	// Cricket Media
	destination_map.insert(
		"Ask 20".to_string(),
		format!("{}__Cricket Media/Ask", BASE_PATH),
	);
	destination_map.insert(
		"Babybug 20".to_string(),
		format!("{}__Cricket Media/Babybug", BASE_PATH),
	);
	destination_map.insert(
		"Click 20".to_string(),
		format!("{}__Cricket Media/Click", BASE_PATH),
	);
	destination_map.insert(
		"Cobblestone 20".to_string(),
		format!("{}__Cricket Media/Cobblestone", BASE_PATH),
	);
	destination_map.insert(
		"Cricket 20".to_string(),
		format!("{}__Cricket Media/Cricket", BASE_PATH),
	);
	destination_map.insert(
		"Faces 20".to_string(),
		format!("{}__Cricket Media/Faces", BASE_PATH),
	);
	destination_map.insert(
		"Ladybug 20".to_string(),
		format!("{}__Cricket Media/Ladybug", BASE_PATH),
	);
	destination_map.insert(
		"Muse 20".to_string(),
		format!("{}__Cricket Media/Muse", BASE_PATH),
	);
	destination_map.insert(
		"Spider 20".to_string(),
		format!("{}__Cricket Media/Spider", BASE_PATH),
	);

	// CSIRO Publishing
	destination_map.insert(
		"Cosmos #".to_string(),
		format!("{}__CSIRO Publishing/Cosmos", BASE_PATH),
	);
	destination_map.insert(
		"Double Helix ".to_string(),
		format!("{}__CSIRO Publishing/Double Helix", BASE_PATH),
	);

	// Culinaire Magazine
	destination_map.insert(
		"Culinaire ".to_string(),
		format!("{}__Culinaire Magazine/Culinaire", BASE_PATH),
	);

	// Cyber Defense Media Group
	destination_map.insert(
		"Cyber Defense Magazine ".to_string(),
		format!("{}__Cyber Defense Media Group/Cyber Defense Magazine", BASE_PATH),
	);

	// D&B Publishing
	destination_map.insert(
		"Pacific PowerBoat ".to_string(),
		format!("{}__D&B Publishing/Pacific PowerBoat", BASE_PATH),
	);

	// Damson Meia
	destination_map.insert(
		"Arts & Collections ".to_string(),
		format!("{}__Damson Meia/Arts & Collections", BASE_PATH),
	);

	// Daphne's Diary BV
	destination_map.insert(
		"Daphne's Diary ".to_string(),
		format!("{}__Daphne's Diary BV/Daphne's Diary", BASE_PATH),
	);

	// David Alderton
	destination_map.insert(
		"Practical Reptile Keeping ".to_string(),
		format!("{}__David Alderton/Practical Reptile Keeping", BASE_PATH),
	);

	// DC Thompson
	destination_map.insert(
		"110% Gaming ".to_string(),
		format!("{}__DC Thompson/110% Gaming", BASE_PATH),
	);
	destination_map.insert(
		"Animal Planet # ".to_string(),
		format!("{}__DC Thompson/Animal Planet", BASE_PATH),
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
		"My Weekly Pocket Novel ".to_string(),
		format!("{}__DC Thompson/My Weekly Pocket Novel", BASE_PATH),
	);
	destination_map.insert(
		"The People's Friend Pocket Novel ".to_string(),
		format!("{}__DC Thompson/The People's Friend Pocket Novel", BASE_PATH),
	);
	destination_map.insert(
		"The Scots Magazine ".to_string(),
		format!("{}__DC Thompson/The Scots Magazine", BASE_PATH),
	);
	destination_map.insert(
		"This England ".to_string(),
		format!("{}__DC Thompson/This England", BASE_PATH),
	);

	// Dead Good Publishing
	destination_map.insert(
		"Haunted Magazine ".to_string(),
		format!("{}__Dead Good Publishing/Haunted Magazine", BASE_PATH),
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
	destination_map.insert(
		"Viz ".to_string(),
		format!("{}__Diamond Publishing/Viz", BASE_PATH),
	);

	// Direct Publishing Pty
	destination_map.insert(
		"Reader's Digest AU ".to_string(),
		format!("{}__Direct Publishing Pty/Reader's Digest AU", BASE_PATH),
	);

	// Discover Your Ancestors
	destination_map.insert(
		"Discover Your Ancestors ".to_string(),
		format!("{}__Discover Your Ancestors/Discover Your Ancestors", BASE_PATH),
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

	// DVV Media International
	destination_map.insert(
		"Flight International ".to_string(),
		format!("{}__DVV Media International/Flight International", BASE_PATH),
	);

	// East Coast Home Publishing
	destination_map.insert(
		"Design + Decor ".to_string(),
		format!("{}__East Coast Home Publishing/Design + Decor", BASE_PATH),
	);

	// EBR Media
	destination_map.insert(
		"The European Business Review ".to_string(),
		format!("{}__EBR Media/The European Business Review", BASE_PATH),
	);

	// Eduard-Model Accessories
	destination_map.insert(
		"Info Eduard ".to_string(),
		format!("{}__Eduard-Model Accessories/Info Eduard", BASE_PATH),
	);

	// EFY Group
	destination_map.insert(
		"Open Source for You ".to_string(),
		format!("{}__EFY Group/Open Source for You", BASE_PATH),
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
	destination_map.insert(
		"Western Life Today ".to_string(),
		format!("{}__EG Media/Western Life Today", BASE_PATH),
	);

	// Electron Publishing
	destination_map.insert(
		"Practical Electronics ".to_string(),
		format!("{}__Electron Publishing/Practical Electronics", BASE_PATH),
	);

	// Elektor International Media
	destination_map.insert(
		"Elektor Mag ".to_string(),
		format!("{}__Elektor International Media/Elektor Mag", BASE_PATH),
	);

	// EM Media
	destination_map.insert(
		"Electronics Maker ".to_string(),
		format!("{}__EM Media/Electronics Maker", BASE_PATH),
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

	// Endeavor Business Media
	destination_map.insert(
		"Architectural Products ".to_string(),
		format!("{}__Endeavor Business Media/Architectural Products", BASE_PATH),
	);
	destination_map.insert(
		"Control Design ".to_string(),
		format!("{}__Endeavor Business Media/Control Design", BASE_PATH),
	);
	destination_map.insert(
		"Locksmith Ledger International ".to_string(),
		format!("{}__Endeavor Business Media/Locksmith Ledger International", BASE_PATH),
	);
	destination_map.insert(
		"Machine Design ".to_string(),
		format!("{}__Endeavor Business Media/Machine Design", BASE_PATH),
	);
	destination_map.insert(
		"Military + Aerospace Electronics ".to_string(),
		format!("{}__Endeavor Business Media/Military + Aerospace Electronics", BASE_PATH),
	);

	// Engaged Media
	destination_map.insert(
		"Knives Illustrated ".to_string(),
		format!("{}__Engaged Media/Knives Illustrated", BASE_PATH),
	);
	destination_map.insert(
		"Tread ".to_string(),
		format!("{}__Engaged Media/Tread", BASE_PATH),
	);

	// Entrepreneur Media
	destination_map.insert(
		"Entrepreneur ".to_string(),
		format!("{}__Entrepreneur Media/Entrepreneur", BASE_PATH),
	);
	destination_map.insert(
		"Entrepreneur's Startups ".to_string(),
		format!("{}__Entrepreneur Media/Entrepreneur's Startups", BASE_PATH),
	);

	// EPS Software Corp
	destination_map.insert(
		"CODE Magazine ".to_string(),
		format!("{}__EPS Software Corp/CODE Magazine", BASE_PATH),
	);

	// __est Magazine
	destination_map.insert(
		"est Living #".to_string(),
		format!("{}__est Magazine/est Living", BASE_PATH),
	);

	// Executive Media Pty
	destination_map.insert(
		"Traces ".to_string(),
		format!("{}__Executive Media Pty/Traces", BASE_PATH),
	);

	// Explore Outdoor Media
	destination_map.insert(
		"Explore 20".to_string(),
		format!("{}__Explore Outdoor Media/Explore", BASE_PATH),
	);

	// Eye to Eye Media
	destination_map.insert(
		"Delicious UK ".to_string(),
		format!("{}__Eye to Eye Media/Delicious UK", BASE_PATH),
	);
	destination_map.insert(
		"Healthy Food Guide UK ".to_string(),
		format!("{}__Eye to Eye Media/Healthy Food Guide UK", BASE_PATH),
	);

	// Falkemedia
	destination_map.insert(
		"Beat #".to_string(),
		format!("{}__Falkemedia/Beat", BASE_PATH),
	);

	// Fieldsports Press
	destination_map.insert(
		"Trout & Salmon ".to_string(),
		format!("{}__Fieldsports Press/Trout & Salmon", BASE_PATH),
	);

	// Fifth Black
	destination_map.insert(
		"Design Anthology Asia ".to_string(),
		format!("{}__Fifth Black/Design Anthology Asia", BASE_PATH),
	);

	// Forbes Media
	destination_map.insert(
		"Forbes 20".to_string(),
		format!("{}__Forbes Media/Forbes", BASE_PATH),
	);
	destination_map.insert(
		"Forbes Asia ".to_string(),
		format!("{}__Forbes Media/Forbes Asia", BASE_PATH),
	);

	// Fortune Media
	destination_map.insert(
		"Fortune 20".to_string(),
		format!("{}__Fortune Media/Fortune", BASE_PATH),
	);
	destination_map.insert(
		"Fortune Asia ".to_string(),
		format!("{}__Fortune Media/Fortune Asia", BASE_PATH),
	);
	destination_map.insert(
		"Fortune EU ".to_string(),
		format!("{}__Fortune Media/Fortune EU", BASE_PATH),
	);

	// FRAME Publishers
	destination_map.insert(
		"Frame #".to_string(),
		format!("{}__FRAME Publishers/Frame", BASE_PATH),
	);

	// France Media Group
	destination_map.insert(
		"France Today ".to_string(),
		format!("{}__France Media Group/France Today", BASE_PATH),
	);
	destination_map.insert(
		"France Property News ".to_string(),
		format!("{}__France Media Group/France Property News", BASE_PATH),
	);

	// France-Amérique Inc
	destination_map.insert(
		"France-Amérique ".to_string(),
		format!("{}__France-Amérique Inc/France-Amérique", BASE_PATH),
	);

	// Fresh Healthy Media
	destination_map.insert(
		"VegNews ".to_string(),
		format!("{}__Fresh Healthy Media/VegNews", BASE_PATH),
	);

	// Full Circle
	destination_map.insert(
		"Full Circle ".to_string(),
		format!("{}__Full Circle/Full Circle", BASE_PATH),
	);

	// Fusion Retro Books
	destination_map.insert(
		"AmtixCPC ".to_string(),
		format!("{}__Fusion Retro Books/AmtixCPC", BASE_PATH),
	);
	destination_map.insert(
		"Crash Micro Action".to_string(),
		format!("{}__Fusion Retro Books/Crash Micro Action", BASE_PATH),
	);
	destination_map.insert(
		"Fusion -".to_string(),
		format!("{}__Fusion Retro Books/Fusion", BASE_PATH),
	);
	destination_map.insert(
		"Fusion #".to_string(),
		format!("{}__Fusion Retro Books/Fusion", BASE_PATH),
	);
	destination_map.insert(
		"Next Magazine -".to_string(),
		format!("{}__Fusion Retro Books/Next Magazine", BASE_PATH),
	);
	destination_map.insert(
		"Next Magazine #".to_string(),
		format!("{}__Fusion Retro Books/Next Magazine", BASE_PATH),
	);
	destination_map.insert(
		"SEGA Force Mega ".to_string(),
		format!("{}__Fusion Retro Books/SEGA Force Mega", BASE_PATH),
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
		"Australian Guitar ".to_string(),
		format!("{}__Future Plc/Australian Guitar", BASE_PATH),
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
		"Digital Photographer #".to_string(),
		format!("{}__Future Plc/Digital Photographer", BASE_PATH),
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
		"Horse & Hound ".to_string(),
		format!("{}__Future Plc/Horse & Hound", BASE_PATH),
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
		"iPhone User Magazine #".to_string(),
		format!("{}__Future Plc/iPhone User Magazine", BASE_PATH),
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

	// GameOn Networking
	destination_map.insert(
		"GameOn ".to_string(),
		format!("{}__GameOn Networking/GameOn", BASE_PATH),
	);

	// Gardner Business Media
	destination_map.insert(
		"Modern Machine Shop ".to_string(),
		format!("{}__Gardner Business Media/Modern Machine Shop", BASE_PATH),
	);

	// Gecko Publishing
	destination_map.insert(
		"Travel Africa ".to_string(),
		format!("{}__Gecko Publishing/Travel Africa", BASE_PATH),
	);

	// Georg
	destination_map.insert(
		"Topos ".to_string(),
		format!("{}__Georg/Topos", BASE_PATH),
	);

	// Ghoulish Publishing
	destination_map.insert(
		"Infinity #".to_string(),
		format!("{}__Ghoulish Publishing/Infinity", BASE_PATH),
	);

	// GIE Media
	destination_map.insert(
		"Aerospace Manufacturing and Design ".to_string(),
		format!("{}__GIE Media/Aerospace Manufacturing and Design", BASE_PATH),
	);

	// Global Aviator
	destination_map.insert(
		"Ultimate Defence ".to_string(),
		format!("{}__Global Aviator/Ultimate Defence", BASE_PATH),
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

	// Graham Digital Holding Company
	destination_map.insert(
		"Foreign Policy ".to_string(),
		format!("{}__Graham Digital Holding Company/Foreign Policy", BASE_PATH),
	);

	// Great Valley Publishing Co.
	destination_map.insert(
		"Today's Dietitian ".to_string(),
		format!("{}__Great Valley Publishing Co./Today's Dietitian", BASE_PATH),
	);

	// Green Press Pty
	destination_map.insert(
		"Green AU ".to_string(),
		format!("{}__Green Press Pty/Green AU", BASE_PATH),
	);

	// Greenlight Publishing
	destination_map.insert(
		"Treasure Hunting ".to_string(),
		format!("{}__Greenlight Publishing/Treasure Hunting", BASE_PATH),
	);

	// Greenspring Media
	destination_map.insert(
		"Real Food ".to_string(),
		format!("{}__Greenspring Media/Real Food", BASE_PATH),
	);

	// Griffin Publishing Solutions
	destination_map.insert(
		"Arthritis Digest ".to_string(),
		format!("{}__Griffin Publishing Solutions/Arthritis Digest", BASE_PATH),
	);

	// GS Media & Events
	destination_map.insert(
		"Wildsam ".to_string(),
		format!("{}__GS Media & Events/Wildsam", BASE_PATH),
	);

	// Guardian News & Media
	destination_map.insert(
		"The Guardian Feast ".to_string(),
		format!("{}__Guardian News & Media/The Guardian Feast", BASE_PATH),
	);
	destination_map.insert(
		"The Guardian Saturday ".to_string(),
		format!("{}__Guardian News & Media/The Guardian Saturday", BASE_PATH),
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

	// Guitar Interactive Limited
	destination_map.insert(
		"Guitar Interactive ".to_string(),
		format!("{}__Guitar Interactive Limited/Guitar Interactive", BASE_PATH),
	);

	// Gun Digest Media
	destination_map.insert(
		"Blade 20".to_string(),
		format!("{}__Gun Digest Media/Blade", BASE_PATH),
	);
	destination_map.insert(
		"Gun Digest ".to_string(),
		format!("{}__Gun Digest Media/Gun Digest", BASE_PATH),
	);

	// GWP Marketing
	destination_map.insert(
		"House & Lifestyle ".to_string(),
		format!("{}__GWP Marketing/House & Lifestyle", BASE_PATH),
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
		"Trail UK".to_string(),
		format!("{}__H Bauer/Trail UK", BASE_PATH),
	);

	// Habari Media
	destination_map.insert(
		"Food & Home Entertaining ".to_string(),
		format!("{}__Habari Media/Food & Home Entertaining", BASE_PATH),
	);
	destination_map.insert(
		"Getaway ".to_string(),
		format!("{}__Habari Media/Getaway", BASE_PATH),
	);
	destination_map.insert(
		"South African Garden & Home ".to_string(),
		format!("{}__Habari Media/South African Garden & Home", BASE_PATH),
	);

	// Hackercool
	destination_map.insert(
		"Hackercool ".to_string(),
		format!("{}__Hackercool/Hackercool", BASE_PATH),
	);

	// Halliker’s Inc
	destination_map.insert(
		"TradersWorld ".to_string(),
		format!("{}__Halliker’s Inc/TradersWorld", BASE_PATH),
	);

	// Happiful
	destination_map.insert(
		"Happiful ".to_string(),
		format!("{}__Happiful/Happiful", BASE_PATH),
	);

	// Harper's Magazine Foundation
	destination_map.insert(
		"Harper's Magazine ".to_string(),
		format!("{}__Harper's Magazine Foundation/Harper's Magazine", BASE_PATH),
	);

	// Harvard Business School Publishing
	destination_map.insert(
		"Harvard Business Review ".to_string(),
		format!("{}__Harvard Business School Publishing/Harvard Business Review", BASE_PATH),
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
		"Elle Decor 20".to_string(),
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
		"Food Network Magazine ".to_string(),
		format!("{}__Hearst/Food Network Magazine", BASE_PATH),
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
		"Men's Health ".to_string(),
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

	// Height of Land Publications
	destination_map.insert(
		"Backcountry ".to_string(),
		format!("{}__Height of Land Publications/Backcountry", BASE_PATH),
	);

	// High Speed Productions
	destination_map.insert(
		"Juxtapoz ".to_string(),
		format!("{}__High Speed Productions/Juxtapoz", BASE_PATH),
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
		"Civil War Times ".to_string(),
		format!("{}__HistoryNet/Civil War Times", BASE_PATH),
	);
	destination_map.insert(
		"Military History ".to_string(),
		format!("{}__HistoryNet/Military History", BASE_PATH),
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

	// Homestyles Media
	destination_map.insert(
		"Home & Design ".to_string(),
		format!("{}__Homestyles Media/Home & Design", BASE_PATH),
	);

	// Hot English Publishing
	destination_map.insert(
		"Learn Hot English ".to_string(),
		format!("{}__Hot English Publishing/Learn Hot English", BASE_PATH),
	);

	// Hudson One Media
	destination_map.insert(
		"Galerie ".to_string(),
		format!("{}__Hudson One Media/Galerie", BASE_PATH),
	);

	// Iceberg Press
	destination_map.insert(
		"The Simple Things ".to_string(),
		format!("{}__Iceberg Press/The Simple Things", BASE_PATH),
	);

	// Iclay Media
	destination_map.insert(
		"Whole Food Living ".to_string(),
		format!("{}__Iclay Media/Whole Food Living", BASE_PATH),
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

	// Idler
	destination_map.insert(
		"The Idler ".to_string(),
		format!("{}__Idler/The Idler", BASE_PATH),
	);

	// IEEE
	destination_map.insert(
		"IEEE Aerospace and Electronic Systems Magazine ".to_string(),
		format!("{}__IEEE/IEEE Aerospace and Electronic Systems Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Antennas & Propagation Magazine ".to_string(),
		format!("{}__IEEE/IEEE Antennas & Propagation Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Circuits and Systems Magazine ".to_string(),
		format!("{}__IEEE/IEEE Circuits and Systems Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Computational Intelligence Magazine ".to_string(),
		format!("{}__IEEE/IEEE Computational Intelligence Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Consumer Electronics Magazine ".to_string(),
		format!("{}__IEEE/IEEE Consumer Electronics Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Electrification Magazine ".to_string(),
		format!("{}__IEEE/IEEE Electrification Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Geoscience and Remote Sensing Magazine ".to_string(),
		format!("{}__IEEE/IEEE Geoscience and Remote Sensing Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Microwave Magazine ".to_string(),
		format!("{}__IEEE/IEEE Microwave Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Potentials ".to_string(),
		format!("{}__IEEE/IEEE Potentials", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Power & Energy Magazine ".to_string(),
		format!("{}__IEEE/IEEE Power & Energy Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Power Electronics Magazine ".to_string(),
		format!("{}__IEEE/IEEE Power Electronics Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Robotics & Automation Magazine ".to_string(),
		format!("{}__IEEE/IEEE Robotics & Automation Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Signal Processing Magazine ".to_string(),
		format!("{}__IEEE/IEEE Signal Processing Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Solid-States Circuits Magazine ".to_string(),
		format!("{}__IEEE/IEEE Solid-States Circuits Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Systems, Man, & Cybernetics Magazine ".to_string(),
		format!("{}__IEEE/IEEE Systems, Man, & Cybernetics Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Technology and Society Magazine ".to_string(),
		format!("{}__IEEE/IEEE Technology and Society Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Vehicular Technology Magazine ".to_string(),
		format!("{}__IEEE/IEEE Vehicular Technology Magazine", BASE_PATH),
	);
	destination_map.insert(
		"IEEE Women in Engineering Magazine ".to_string(),
		format!("{}__IEEE/IEEE Women in Engineering Magazine", BASE_PATH),
	);
	destination_map.insert(
		"The Bridge 20".to_string(),
		format!("{}__IEEE/The Bridge", BASE_PATH),
	);

	// IMF
	destination_map.insert(
		"Finance & Development ".to_string(),
		format!("{}__IMF/Finance & Development", BASE_PATH),
	);

	// IMI Group
	destination_map.insert(
		"Electronic Product Design & Test ".to_string(),
		format!("{}__IMI Group/Electronic Product Design & Test", BASE_PATH),
	);

	// Immediate
	destination_map.insert(
		"BBC Countryfile ".to_string(),
		format!("{}__Immediate/BBC Countryfile", BASE_PATH),
	);
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
		"BBC Good Food -".to_string(),
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
		"BBC Sky at Night ".to_string(),
		format!("{}__Immediate/BBC Sky at Night", BASE_PATH),
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

	// Inbound Logistics
	destination_map.insert(
		"Inbound Logistics ".to_string(),
		format!("{}__Inbound Logistics/Inbound Logistics", BASE_PATH),
	);

	// Indesign Media Asia Pacific
	destination_map.insert(
		"Habitus #".to_string(),
		format!("{}__Indesign Media Asia Pacific/Habitus", BASE_PATH),
	);
	destination_map.insert(
		"INDESIGN ".to_string(),
		format!("{}__Indesign Media Asia Pacific/INDESIGN", BASE_PATH),
	);

	// Information Today
	destination_map.insert(
		"Computers in Libraries ".to_string(),
		format!("{}__Information Today/Computers in Libraries", BASE_PATH),
	);

	// Innovative Properties Worldwide Inc
	destination_map.insert(
		"Innovation & Tech Today ".to_string(),
		format!("{}__Innovative Properties Worldwide Inc/Innovation & Tech Today", BASE_PATH),
	);
	destination_map.insert(
		"Residential Tech Today ".to_string(),
		format!("{}__Innovative Properties Worldwide Inc/Residential Tech Today", BASE_PATH),
	);

	// Interiors Media
	destination_map.insert(
		"Interiors Monthly ".to_string(),
		format!("{}__Interiors Media/Interiors Monthly", BASE_PATH),
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

	// iQ Business Media
	destination_map.insert(
		"Canadian Architect ".to_string(),
		format!("{}__iQ Business Media/Canadian Architect", BASE_PATH),
	);
	destination_map.insert(
		"Canadian Interiors ".to_string(),
		format!("{}__iQ Business Media/Canadian Interiors", BASE_PATH),
	);

	// Irresistible
	destination_map.insert(
		"Retro Pop ".to_string(),
		format!("{}__Irresistible/Retro Pop", BASE_PATH),
	);

	// Islander Yachting Media
	destination_map.insert(
		"The Islander Magazine ".to_string(),
		format!("{}__Islander Yachting Media/The Islander Magazine", BASE_PATH),
	);

	// Jacobin Foundation
	destination_map.insert(
		"Jacobin #".to_string(),
		format!("{}__Jacobin Foundation/Jacobin", BASE_PATH),
	);

	// JAM Consulting & Design
	destination_map.insert(
		"Antiques to Vintage ".to_string(),
		format!("{}__JAM Consulting & Design/Antiques to Vintage", BASE_PATH),
	);

	// Jeanne d’Arc Living
	destination_map.insert(
		"Jeanne d'Arc Living ".to_string(),
		format!("{}__Jeanne d’Arc Living/Jeanne d'Arc Living", BASE_PATH),
	);

	// Jungo TV
	destination_map.insert(
		"Black Belt ".to_string(),
		format!("{}__Jungo TV/Black Belt", BASE_PATH),
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

	// Kantos Media
	destination_map.insert(
		"Reclaim #".to_string(),
		format!("{}__Kantos Media/Reclaim", BASE_PATH),
	);

	// Kappa Publishing Group
	destination_map.insert(
		"Games World of Puzzles ".to_string(),
		format!("{}__Kappa Publishing Group/Games World of Puzzles", BASE_PATH),
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
		format!(
			"{}__Karwansaray Publishers/Wargames Soldiers & Strategy Magazine",
			BASE_PATH
		),
	);

	// Kaye Publiishing Corp
	destination_map.insert(
		"Graphic Design USA ".to_string(),
		format!("{}__Kaye Publiishing Corp/Graphic Design USA", BASE_PATH),
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
		"Sea Angler ".to_string(),
		format!("{}__Kelsey/Sea Angler", BASE_PATH),
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
		"Vineyard ".to_string(),
		format!("{}__Kelsey/Vineyard", BASE_PATH),
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

	// Kenilworth Media
	destination_map.insert(
		"Metal Architecture ".to_string(),
		format!("{}__Kenilworth Media/Metal Architecture", BASE_PATH),
	);

	// Kennedy Publishing
	destination_map.insert(
		"Extinct #".to_string(),
		format!("{}__Kennedy Publishing/Extinct", BASE_PATH),
	);
	destination_map.insert(
		"Kraze #".to_string(),
		format!("{}__Kennedy Publishing/Kraze", BASE_PATH),
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
		"Classic Land Rover ".to_string(),
		format!("{}__Key/Classic Land Rover", BASE_PATH),
	);
	destination_map.insert(
		"Combat Aircraft ".to_string(),
		format!("{}__Key/Combat Aircraft", BASE_PATH),
	);
	destination_map.insert(
		"FlyPast ".to_string(),
		format!("{}__Key/FlyPast", BASE_PATH),
	);

	// Kickin' Cuts
	destination_map.insert(
		"Country Music People ".to_string(),
		format!("{}__Kickin' Cuts/Country Music People", BASE_PATH),
	);

	// Koru Media
	destination_map.insert(
		"ABCD ".to_string(),
		format!("{}__Koru Media/ABCD", BASE_PATH),
	);

	// L2 Architectural Media
	destination_map.insert(
		"The Architectural Technologists Book ".to_string(),
		format!("{}__L2 Architectural Media/The Architectural Technologists Book", BASE_PATH),
	);
	destination_map.insert(
		"Building Innovations ".to_string(),
		format!("{}__L2 Architectural Media/Building Innovations", BASE_PATH),
	);

	// L'Officiel
	destination_map.insert(
		"L'Officiel ".to_string(),
		format!("{}__L'Officiel", BASE_PATH),
	);

	// LabX Media Group
	destination_map.insert(
		"The Scientist ".to_string(),
		format!("{}__LabX Media Group/The Scientist", BASE_PATH),
	);

	// Lake Avenue Publishing
	destination_map.insert(
		"The New Republic ".to_string(),
		format!("{}__Lake Avenue Publishing/The New Republic", BASE_PATH),
	);

	// LE Publications
	destination_map.insert(
		"Life Extension ".to_string(),
		format!("{}__LE Publications/Life Extension", BASE_PATH),
	);

	// Lifestyle Publishing
	destination_map.insert(
		"Wilderness 20".to_string(),
		format!("{}__Lifestyle Publishing/Wilderness", BASE_PATH),
	);

	// Linux New Media
	destination_map.insert(
		"ADMIN Magazine ".to_string(),
		format!("{}__Linux New Media/ADMIN Magazine", BASE_PATH),
	);
	destination_map.insert(
		"LibreOffice Expert ".to_string(),
		format!("{}__Linux New Media/LibreOffice Expert", BASE_PATH),
	);
	destination_map.insert(
		"Linux Magazine ".to_string(),
		format!("{}__Linux New Media/Linux Magazine", BASE_PATH),
	);
	destination_map.insert(
		"MakerSpace ".to_string(),
		format!("{}__Linux New Media/MakerSpace", BASE_PATH),
	);

	// Literary Review of Canada Charitable Foundation
	destination_map.insert(
		"Literary Review of Canada ".to_string(),
		format!(
			"{}__Literary Review of Canada Charitable Foundation/Literary Review of Canada",
			BASE_PATH
		),
	);

	// Locomotive 6960
	destination_map.insert(
		"The Critic ".to_string(),
		format!("{}__Locomotive 6960/The Critic", BASE_PATH),
	);

	// Lonehill Media
	destination_map.insert(
		"Grow to Eat ".to_string(),
		format!("{}__Lonehill Media/Grow to Eat", BASE_PATH),
	);

	// Lovatts Media
	destination_map.insert(
		"Lovatts Handy Arrowords ".to_string(),
		format!("{}__Lovatts Media/Lovatts Handy Arrowords", BASE_PATH),
	);
	destination_map.insert(
		"Lovatts Handy Crosswords ".to_string(),
		format!("{}__Lovatts Media/Lovatts Handy Crosswords", BASE_PATH),
	);
	destination_map.insert(
		"Lovatts Handy Sudoku ".to_string(),
		format!("{}__Lovatts Media/Lovatts Handy Sudoku", BASE_PATH),
	);
	destination_map.insert(
		"Mindful Puzzles ".to_string(),
		format!("{}__Lovatts Media/Mindful Puzzles", BASE_PATH),
	);
	destination_map.insert(
		"Nourish Plant-Based Living ".to_string(),
		format!("{}__Lovatts Media/Nourish Plant-Based Living", BASE_PATH),
	);

	// Madavor Media
	destination_map.insert(
		"BirdWatching US ".to_string(),
		format!("{}__Madavor Media/BirdWatching US", BASE_PATH),
	);
	destination_map.insert(
		"Diabetes Self-Management ".to_string(),
		format!("{}__Madavor Media/Diabetes Self-Management", BASE_PATH),
	);
	destination_map.insert(
		"The Writer ".to_string(),
		format!("{}__Madavor Media/The Writer", BASE_PATH),
	);

	// Magazine Antiques Media
	destination_map.insert(
		"The Magazine Antiques ".to_string(),
		format!("{}__Magazine Antiques Media/The Magazine Antiques", BASE_PATH),
	);

	// Magazine Design & Publishing
	destination_map.insert(
		"True Crime ".to_string(),
		format!("{}__Magazine Design & Publishing/True Crime", BASE_PATH),
	);
	destination_map.insert(
		"True Detective ".to_string(),
		format!("{}__Magazine Design & Publishing/True Detective", BASE_PATH),
	);

	// Maher Publications
	destination_map.insert(
		"Downbeat ".to_string(),
		format!("{}__Maher Publications/Downbeat", BASE_PATH),
	);

	// Make Community
	destination_map.insert(
		"Make #".to_string(),
		format!("{}__Make Community/Make", BASE_PATH),
	);

	// Man Wees Pty
	destination_map.insert(
		"Global Aviator ".to_string(),
		format!("{}__Man Wees Pty/Global Aviator", BASE_PATH),
	);

	// Mandelgren Magazine
	destination_map.insert(
		"FORM ".to_string(),
		format!("{}__Mandelgren Magazine/FORM", BASE_PATH),
	);

	// Mango Life Media
	destination_map.insert(
		"iPhone Life ".to_string(),
		format!("{}__Mango Life Media/iPhone Life", BASE_PATH),
	);

	// Mansueto Ventures
	destination_map.insert(
		"Fast Company ".to_string(),
		format!("{}__Mansueto Ventures/Fast Company", BASE_PATH),
	);
	destination_map.insert(
		"Inc. ".to_string(),
		format!("{}__Mansueto Ventures/Inc.", BASE_PATH),
	);

	// Marine Media
	destination_map.insert(
		"Boating NZ ".to_string(),
		format!("{}__Marine Media/Boating NZ", BASE_PATH),
	);

	// Mark Allen Group
	destination_map.insert(
		"Aerospace Testing International ".to_string(),
		format!("{}__Mark Allen Group/Aerospace Testing International", BASE_PATH),
	);
	destination_map.insert(
		"Choir & Organ ".to_string(),
		format!("{}__Mark Allen Group/Choir & Organ", BASE_PATH),
	);
	destination_map.insert(
		"Electric & Hybrid Vehicle Technology International ".to_string(),
		format!(
			"{}__Mark Allen Group/Electric & Hybrid Vehicle Technology International",
			BASE_PATH
		),
	);
	destination_map.insert(
		"Eureka! ".to_string(),
		format!("{}__Mark Allen Group/Eureka!", BASE_PATH),
	);
	destination_map.insert(
		"Gramophone ".to_string(),
		format!("{}__Mark Allen Group/Gramophone", BASE_PATH),
	);
	destination_map.insert(
		"International Piano ".to_string(),
		format!("{}__Mark Allen Group/International Piano", BASE_PATH),
	);
	destination_map.insert(
		"Intertraffic World ".to_string(),
		format!("{}__Mark Allen Group/Intertraffic World", BASE_PATH),
	);
	destination_map.insert(
		"Jazzwise ".to_string(),
		format!("{}__Mark Allen Group/Jazzwise", BASE_PATH),
	);
	destination_map.insert(
		"Machinery ".to_string(),
		format!("{}__Mark Allen Group/Machinery", BASE_PATH),
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
	destination_map.insert(
		"Traffic Technology International ".to_string(),
		format!("{}__Mark Allen Group/Traffic Technology International", BASE_PATH),
	);

	// Massachusetts Institute of Technology
	destination_map.insert(
		"MIT Sloan Management Review ".to_string(),
		format!(
			"{}__Massachusetts Institute of Technology/MIT Sloan Management Review",
			BASE_PATH
		),
	);

	// Media 10
	destination_map.insert(
		"Good Homes 20".to_string(),
		format!("{}__Media 10/Good Homes", BASE_PATH),
	);
	destination_map.insert(
		"Grand Designs ".to_string(),
		format!("{}__Media 10/Grand Designs", BASE_PATH),
	);
	destination_map.insert(
		"ICON ".to_string(),
		format!("{}__Media 10/ICON", BASE_PATH),
	);
	destination_map.insert(
		"OnOffice ".to_string(),
		format!("{}__Media 10/OnOffice", BASE_PATH),
	);

	// Media 24
	destination_map.insert(
		"Air-Fryer ".to_string(),
		format!("{}__Media 24/Air-Fryer", BASE_PATH),
	);
	destination_map.insert(
		"Home Quick Food ".to_string(),
		format!("{}__Media 24/Home Quick Food", BASE_PATH),
	);
	destination_map.insert(
		"Home SA ".to_string(),
		format!("{}__Media 24/Home SA", BASE_PATH),
	);
	destination_map.insert(
		"Lose It! ".to_string(),
		format!("{}__Media 24/Lose It!", BASE_PATH),
	);
	destination_map.insert(
		"Man Magnum ".to_string(),
		format!("{}__Media 24/Man Magnum", BASE_PATH),
	);
	destination_map.insert(
		"Visi ".to_string(),
		format!("{}__Media 24/Visi", BASE_PATH),
	);
	destination_map.insert(
		"Woolworths Taste ".to_string(),
		format!("{}__Media 24/Woolworths Taste", BASE_PATH),
	);

	// Media Group PTE
	destination_map.insert(
		"Design + Architecture ".to_string(),
		format!("{}__Media Group PTE/Design + Architecture", BASE_PATH),
	);
	destination_map.insert(
		"Epicure SG ".to_string(),
		format!("{}__Media Group PTE/Epicure SG", BASE_PATH),
	);
	destination_map.insert(
		"SquareRooms ".to_string(),
		format!("{}__Media Group PTE/SquareRooms", BASE_PATH),
	);

	// Media Transasia
	destination_map.insert(
		"Armada International ".to_string(),
		format!("{}__Media Transasia/Armada International", BASE_PATH),
	);

	// Media Xpose
	destination_map.insert(
		"To Build ".to_string(),
		format!("{}__Media Xpose/To Build", BASE_PATH),
	);

	// MediaOne
	destination_map.insert(
		"Architecture & Design ".to_string(),
		format!("{}__MediaOne/Architecture & Design", BASE_PATH),
	);
	destination_map.insert(
		"Architecture Magazine ".to_string(),
		format!("{}__MediaOne/Architecture Magazine", BASE_PATH),
	);
	destination_map.insert(
		"Interior Designer ".to_string(),
		format!("{}__MediaOne/Interior Designer", BASE_PATH),
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
		"Country French ".to_string(),
		format!("{}__Meredith/Country French", BASE_PATH),
	);
	destination_map.insert(
		"Country Home ".to_string(),
		format!("{}__Meredith/Country Home", BASE_PATH),
	);
	destination_map.insert(
		"EatingWell ".to_string(),
		format!("{}__Meredith/EatingWell", BASE_PATH),
	);
	destination_map.insert(
		"Entertainment Weekly ".to_string(),
		format!("{}__Meredith/Entertainment Weekly", BASE_PATH),
	);
	destination_map.insert(
		"Food & Wine ".to_string(),
		format!("{}__Meredith/Food & Wine", BASE_PATH),
	);
	destination_map.insert(
		"Forks Over Knives ".to_string(),
		format!("{}__Meredith/Forks Over Knives", BASE_PATH),
	);
	destination_map.insert(
		"Midwest Living ".to_string(),
		format!("{}__Meredith/Midwest Living", BASE_PATH),
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
		"Southern Living ".to_string(),
		format!("{}__Meredith/Southern Living", BASE_PATH),
	);
	destination_map.insert(
		"Successful Farming ".to_string(),
		format!("{}__Meredith/Successful Farming", BASE_PATH),
	);
	destination_map.insert(
		"Traditional Home ".to_string(),
		format!("{}__Meredith/Traditional Home", BASE_PATH),
	);

	// Metros Publishing Group NZ
	destination_map.insert(
		"Metropol ".to_string(),
		format!("{}__Metros Publishing Group NZ/Metropol", BASE_PATH),
	);

	// MH Media
	destination_map.insert(
		"Design Buy Build ".to_string(),
		format!("{}__MH Media/Design Buy Build", BASE_PATH),
	);
	destination_map.insert(
		"Landscape & Urban Design ".to_string(),
		format!("{}__MH Media/Landscape & Urban Design", BASE_PATH),
	);
	destination_map.insert(
		"Refurb & Restore ".to_string(),
		format!("{}__MH Media/Refurb & Restore", BASE_PATH),
	);
	destination_map.insert(
		"The Art of Design ".to_string(),
		format!("{}__MH Media/The Art of Design", BASE_PATH),
	);
	destination_map.insert(
		"The Art of Luxury ".to_string(),
		format!("{}__MH Media/The Art of Luxury", BASE_PATH),
	);

	// Milne Publishing
	destination_map.insert(
		"Artist Talk Magazine ".to_string(),
		format!("{}__Milne Publishing/Artist Talk Magazine", BASE_PATH),
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

	// MIT Publishing
	destination_map.insert(
		"Aerospace Manufacturing Magazine ".to_string(),
		format!("{}__MIT Publishing/Aerospace Manufacturing Magazine", BASE_PATH),
	);

	// MIT Technology Review
	destination_map.insert(
		"MIT Technology Review ".to_string(),
		format!("{}__MIT Technology Review/MIT Technology Review", BASE_PATH),
	);

	// MJC Digital Pty
	destination_map.insert(
		"Eat. Live. Escape. ".to_string(),
		format!("{}__MJC Digital Pty/Eat. Live. Escape.", BASE_PATH),
	);
	destination_map.insert(
		"ele HOME ".to_string(),
		format!("{}__MJC Digital Pty/ele HOME", BASE_PATH),
	);

	// Modeliste
	destination_map.insert(
		"Modeliste ".to_string(),
		format!("{}__Modeliste/Modeliste", BASE_PATH),
	);

	// Modern Drummer Publications
	destination_map.insert(
		"Modern Drummer ".to_string(),
		format!("{}__Modern Drummer Publications/Modern Drummer", BASE_PATH),
	);

	// Mondiale Publishing
	destination_map.insert(
		"Arc #".to_string(),
		format!("{}__Mondiale Publishing/Arc", BASE_PATH),
	);
	destination_map.insert(
		"Darc #".to_string(),
		format!("{}__Mondiale Publishing/Darc", BASE_PATH),
	);

	// Moorshead Magazines
	destination_map.insert(
		"History Magazine Quarterly ".to_string(),
		format!("{}__Moorshead Magazines/History Magazine Quarterly", BASE_PATH),
	);
	destination_map.insert(
		"Internet Genealogy ".to_string(),
		format!("{}__Moorshead Magazines/Internet Genealogy", BASE_PATH),
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

	// Motivate Media Group
	destination_map.insert(
		"Identity #".to_string(),
		format!("{}__Motivate Media Group/Identity", BASE_PATH),
	);

	// Music Connection Media
	destination_map.insert(
		"Music Connection ".to_string(),
		format!("{}__Music Connection Media/Music Connection", BASE_PATH),
	);

	// MyTime Media
	destination_map.insert(
		"Hi-Fi Choice ".to_string(),
		format!("{}__MyTime Media/Hi-Fi Choice", BASE_PATH),
	);
	destination_map.insert(
		"Hi-Fi News ".to_string(),
		format!("{}__MyTime Media/Hi-Fi News", BASE_PATH),
	);
	destination_map.insert(
		"Home Cinema Choice ".to_string(),
		format!("{}__MyTime Media/Home Cinema Choice", BASE_PATH),
	);
	destination_map.insert(
		"Sound & Vision ".to_string(),
		format!("{}__MyTime Media/Sound & Vision", BASE_PATH),
	);
	destination_map.insert(
		"Stamp Magazine ".to_string(),
		format!("{}__MyTime Media/Stamp Magazine", BASE_PATH),
	);
	destination_map.insert(
		"Stereophile ".to_string(),
		format!("{}__MyTime Media/Stereophile", BASE_PATH),
	);

	// NAAA
	destination_map.insert(
		"Agricultural Aviation ".to_string(),
		format!("{}__NAAA/Agricultural Aviation", BASE_PATH),
	);

	// National Audubon Society
	destination_map.insert(
		"Audubon ".to_string(),
		format!("{}__National Audubon Society/Audubon", BASE_PATH),
	);

	// National Defense Industrial Association
	destination_map.insert(
		"National Defense ".to_string(),
		format!("{}__National Defense Industrial Association/National Defense", BASE_PATH),
	);

	// National Geographic
	destination_map.insert(
		"National Geographic 20".to_string(),
		format!("{}__National Geographic/National Geographic", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic History ".to_string(),
		format!("{}__National Geographic/National Geographic History", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic Kids 20".to_string(),
		format!("{}__National Geographic/National Geographic Kids", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic Kids ANZ ".to_string(),
		format!("{}__National Geographic/National Geographic Kids ANZ", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic Kids UK".to_string(),
		format!("{}__National Geographic/National Geographic Kids UK", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic Little Kids ".to_string(),
		format!("{}__National Geographic/National Geographic Little Kids", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic Traveller 20".to_string(),
		format!("{}__National Geographic/National Geographic Traveller", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic Traveller -".to_string(),
		format!("{}__National Geographic/National Geographic Traveller", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic Traveller Food ".to_string(),
		format!("{}__National Geographic/National Geographic Traveller Food", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic Traveller UK ".to_string(),
		format!("{}__National Geographic/National Geographic Traveller UK", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic Traveller UK ".to_string(),
		format!("{}__National Geographic/National Geographic Traveller UK", BASE_PATH),
	);
	destination_map.insert(
		"National Geographic UK ".to_string(),
		format!("{}__National Geographic/National Geographic UK", BASE_PATH),
	);

	// National Maritime Historical Society
	destination_map.insert(
		"Sea History ".to_string(),
		format!("{}__National Maritime Historical Society/Sea History", BASE_PATH),
	);

	// National Review Inc
	destination_map.insert(
		"National Review ".to_string(),
		format!("{}__National Review Inc/National Review", BASE_PATH),
	);

	// National Wildlife Federation USA
	destination_map.insert(
		"National Wildlife ".to_string(),
		format!("{}__National Wildlife Federation USA/National Wildlife", BASE_PATH),
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

	// New Horizon
	destination_map.insert(
		"Scale Aviation & Military Modeller International ".to_string(),
		format!("{}__New Horizon/Scale Aviation & Military Modeller International", BASE_PATH),
	);

	// New Internationalist Publications
	destination_map.insert(
		"New Internationalist ".to_string(),
		format!("{}__New Internationalist Publications/New Internationalist", BASE_PATH),
	);

	// New Scientist
	destination_map.insert(
		"New Scientist ".to_string(),
		format!("{}__New Scientist/New Scientist", BASE_PATH),
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

	// Newsline Publications Pvt
	destination_map.insert(
		"Geopolitics ".to_string(),
		format!("{}__Newsline Publications Pvt/Geopolitics", BASE_PATH),
	);

	// Newsquest
	destination_map.insert(
		"The Strad ".to_string(),
		format!("{}__Newsquest/The Strad", BASE_PATH),
	);

	// Newsweek
	destination_map.insert(
		"Newsweek ".to_string(),
		format!("{}__Newsweek/Newsweek", BASE_PATH),
	);

	// NextHome
	destination_map.insert(
		"Reno + Decor ".to_string(),
		format!("{}__NextHome/Reno + Decor", BASE_PATH),
	);

	// Nextmedia
	destination_map.insert(
		"ABC Organic Gardener ".to_string(),
		format!("{}__Nextmedia/ABC Organic Gardener", BASE_PATH),
	);
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
		"K-Zone ".to_string(),
		format!("{}__Nextmedia/K-Zone", BASE_PATH),
	);
	destination_map.insert(
		"Science Illustrated AU ".to_string(),
		format!("{}__Nextmedia/Science Illustrated AU", BASE_PATH),
	);

	// Nextscreen
	destination_map.insert(
		"The Absolute Sound ".to_string(),
		format!("{}__Nextscreen/The Absolute Sound", BASE_PATH),
	);

	// NFU
	destination_map.insert(
		"Countryside 20".to_string(),
		format!("{}__NFU/Countryside", BASE_PATH),
	);

	// Niche Media Pty
	destination_map.insert(
		"Architectural Review Asia Pacific ".to_string(),
		format!("{}__Niche Media Pty/Architectural Review Asia Pacific", BASE_PATH),
	);
	destination_map.insert(
		"Inside Interior Design Review ".to_string(),
		format!("{}__Niche Media Pty/Inside Interior Design Review", BASE_PATH),
	);

	// Ninty Media
	destination_map.insert(
		"Switch Player ".to_string(),
		format!("{}__Ninty Media/Switch Player", BASE_PATH),
	);

	// Nook Publishing
	destination_map.insert(
		"Home NZ ".to_string(),
		format!("{}__Nook Publishing/Home NZ", BASE_PATH),
	);

	// North Co
	destination_map.insert(
		"Artful Living ".to_string(),
		format!("{}__North Co/Artful Living", BASE_PATH),
	);

	// North Coast Media
	destination_map.insert(
		"GPS World ".to_string(),
		format!("{}__North Coast Media/GPS World", BASE_PATH),
	);

	// Nuclear Media
	destination_map.insert(
		"Krash #".to_string(),
		format!("{}__Nuclear Media/Krash", BASE_PATH),
	);

	// NYREV
	destination_map.insert(
		"The New York Review of Books ".to_string(),
		format!("{}__NYREV/The New York Review of Books", BASE_PATH),
	);

	// Ogden Publications Inc
	destination_map.insert(
		"Grit Magazine ".to_string(),
		format!("{}__Ogden Publications Inc/Grit Magazine", BASE_PATH),
	);

	// OP Media Group
	destination_map.insert(
		"Pacific Yachting ".to_string(),
		format!("{}__OP Media Group/Pacific Yachting", BASE_PATH),
	);

	// OpenSystems Media
	destination_map.insert(
		"Embedded Computing Design ".to_string(),
		format!("{}__OpenSystems Media/Embedded Computing Design", BASE_PATH),
	);

	// Outdoor Sportsman Group
	destination_map.insert(
		"Firearms News ".to_string(),
		format!("{}__Outdoor Sportsman Group/Firearms News", BASE_PATH),
	);
	destination_map.insert(
		"Florida Sportsman ".to_string(),
		format!("{}__Outdoor Sportsman Group/Florida Sportsman", BASE_PATH),
	);
	destination_map.insert(
		"Guns & Ammo ".to_string(),
		format!("{}__Outdoor Sportsman Group/Guns & Ammo", BASE_PATH),
	);
	destination_map.insert(
		"Handguns 20".to_string(),
		format!("{}__Outdoor Sportsman Group/Handguns", BASE_PATH),
	);

	// Outside Inc
	destination_map.insert(
		"Backpacker ".to_string(),
		format!("{}__Outside Inc/Backpacker", BASE_PATH),
	);
	destination_map.insert(
		"National Park Journal ".to_string(),
		format!("{}__Outside Inc/National Park Journal", BASE_PATH),
	);
	destination_map.insert(
		"Outside ".to_string(),
		format!("{}__Outside Inc/Outside", BASE_PATH),
	);
	destination_map.insert(
		"Yoga Journal ".to_string(),
		format!("{}__Outside Inc/Yoga Journal", BASE_PATH),
	);

	// Page One Publishing
	destination_map.insert(
		"YAM Magazine ".to_string(),
		format!("{}__Page One Publishing/YAM Magazine", BASE_PATH),
	);

	// Palm Beach Media Group
	destination_map.insert(
		"Art & Culture ".to_string(),
		format!("{}__Palm Beach Media Group/Art & Culture", BASE_PATH),
	);
	destination_map.insert(
		"Florida Design ".to_string(),
		format!("{}__Palm Beach Media Group/Florida Design", BASE_PATH),
	);

	// Pam Communications
	destination_map.insert(
		"Electronic Sound ".to_string(),
		format!("{}__Pam Communications/Electronic Sound", BASE_PATH),
	);

	// Panorama Media Corp
	destination_map.insert(
		"MyKitchen SA ".to_string(),
		format!("{}__Panorama Media Corp/MyKitchen SA", BASE_PATH),
	);
	destination_map.insert(
		"Very Interesting ".to_string(),
		format!("{}__Panorama Media Corp/Very Interesting", BASE_PATH),
	);

	// Paragraph Publishing
	destination_map.insert(
		"Whisky Magazine ".to_string(),
		format!("{}__Paragraph Publishing/Whisky Magazine", BASE_PATH),
	);

	// Parkerson Publishing
	destination_map.insert(
		"Astronomy Technology Today ".to_string(),
		format!("{}__Parkerson Publishing/Astronomy Technology Today", BASE_PATH),
	);

	// PCL
	destination_map.insert(
		"PCL's ".to_string(),
		format!("{}__PCL/_MagBooks", BASE_PATH),
	);
	destination_map.insert(
		"Essential Apple User ".to_string(),
		format!("{}__PCL/Essential Apple User", BASE_PATH),
	);
	destination_map.insert(
		"iPhone User #".to_string(),
		format!("{}__PCL/iPhone User", BASE_PATH),
	);
	destination_map.insert(
		"Mac + MacBook User ".to_string(),
		format!("{}__PCL/Mac + MacBook User", BASE_PATH),
	);

	// Peninsula Publishing
	destination_map.insert(
		"The Local Palate ".to_string(),
		format!("{}__Peninsula Publishing/The Local Palate", BASE_PATH),
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

	// Peppermint Magazine
	destination_map.insert(
		"Peppermint #".to_string(),
		format!("{}__Peppermint Magazine/Peppermint", BASE_PATH),
	);

	// Perspective Magazine
	destination_map.insert(
		"Perspective UK ".to_string(),
		format!("{}__Perspective Magazine/Perspective UK", BASE_PATH),
	);

	// Phi Beta Kappa Society
	destination_map.insert(
		"The American Scholar ".to_string(),
		format!("{}__Phi Beta Kappa Society/The American Scholar", BASE_PATH),
	);

	// Phoenix Scale Publications
	destination_map.insert(
		"Phoenix Aviation Modelling ".to_string(),
		format!("{}__Phoenix Scale Publications/Phoenix Aviation Modelling", BASE_PATH),
	);

	// Pierce Publishing
	destination_map.insert(
		"Milieu ".to_string(),
		format!("{}__Pierce Publishing/Milieu", BASE_PATH),
	);

	// Pip Magazine
	destination_map.insert(
		"Pip Magazine ".to_string(),
		format!("{}__Pip Magazine/Pip Magazine", BASE_PATH),
	);

	// Poet Press
	destination_map.insert(
		"New Philosopher ".to_string(),
		format!("{}__Poet Press/New Philosopher", BASE_PATH),
	);

	// Poets & Writers
	destination_map.insert(
		"Poets & Writers ".to_string(),
		format!("{}__Poets & Writers/Poets & Writers", BASE_PATH),
	);

	// Powerboat and RIB
	destination_map.insert(
		"Powerboat & RIB ".to_string(),
		format!("{}__Powerboat and RIB/Powerboat & RIB", BASE_PATH),
	);

	// Prehistoric Times
	destination_map.insert(
		"Prehistoric Times ".to_string(),
		format!("{}__Prehistoric Times/Prehistoric Times", BASE_PATH),
	);

	// Pressdram
	destination_map.insert(
		"Private Eye ".to_string(),
		format!("{}__Pressdram/Private Eye", BASE_PATH),
	);

	// Prime Creative Media
	destination_map.insert(
		"BeanScene ".to_string(),
		format!("{}__Prime Creative Media/BeanScene", BASE_PATH),
	);
	destination_map.insert(
		"Farms & Farm Machinery ".to_string(),
		format!("{}__Prime Creative Media/Farms & Farm Machinery", BASE_PATH),
	);

	// Prime Impact
	destination_map.insert(
		"OM Yoga & Lifestyle ".to_string(),
		format!("{}__Prime Impact/OM Yoga & Lifestyle", BASE_PATH),
	);

	// Print Ideas
	destination_map.insert(
		"Art Guide AU ".to_string(),
		format!("{}__Print Ideas/Art Guide AU", BASE_PATH),
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

	// Prospect Publishing
	destination_map.insert(
		"Prospect 20".to_string(),
		format!("{}__Prospect Publishing/Prospect", BASE_PATH),
	);

	// Puzzler Media
	destination_map.insert(
		"Premium Crosswords ".to_string(),
		format!("{}__Puzzler Media/Premium Crosswords", BASE_PATH),
	);
	destination_map.insert(
		"Premium Sudoku Puzzles ".to_string(),
		format!("{}__Puzzler Media/Premium Sudoku Puzzles", BASE_PATH),
	);

	// PWxyz
	destination_map.insert(
		"Publishers Weekly ".to_string(),
		format!("{}__PWxyz/Publishers Weekly", BASE_PATH),
	);

	// Rainmaker Information
	destination_map.insert(
		"Money AU ".to_string(),
		format!("{}__Rainmaker Information/Money AU", BASE_PATH),
	);

	// ramp.space
	destination_map.insert(
		"Ramp #".to_string(),
		format!("{}__ramp.space/Ramp", BASE_PATH),
	);
	destination_map.insert(
		"Rampstyle ".to_string(),
		format!("{}__ramp.space/Rampstyle", BASE_PATH),
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

	// Reader’s Digest Magazines Canada
	destination_map.insert(
		"More of Our Canada ".to_string(),
		format!("{}__Reader’s Digest Magazines Canada/More of Our Canada", BASE_PATH),
	);
	destination_map.insert(
		"Our Canada".to_string(),
		format!("{}__Reader’s Digest Magazines Canada/Our Canada", BASE_PATH),
	);
	destination_map.insert(
		"Reader's Digest CA ".to_string(),
		format!("{}__Reader’s Digest Magazines Canada/Reader's Digest CA", BASE_PATH),
	);

	// Reason
	destination_map.insert(
		"Reason 20".to_string(),
		format!("{}__Reason/Reason", BASE_PATH),
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

	// Renew
	destination_map.insert(
		"Renew #".to_string(),
		format!("{}__Renew/Renew", BASE_PATH),
	);
	destination_map.insert(
		"Sanctuary Modern Green Homes ".to_string(),
		format!("{}__Renew/Sanctuary Modern Green Homes", BASE_PATH),
	);

	// revue Argument
	destination_map.insert(
		"Argument 20".to_string(),
		format!("{}__revue Argument/Argument", BASE_PATH),
	);

	// RMS Media Group
	destination_map.insert(
		"Ocean Home 20".to_string(),
		format!("{}__RMS Media Group/Ocean Home", BASE_PATH),
	);

	// Rock Sound
	destination_map.insert(
		"Rock Sound ".to_string(),
		format!("{}__Rock Sound/Rock Sound", BASE_PATH),
	);

	// Rork Media
	destination_map.insert(
		"Scuba Diver ".to_string(),
		format!("{}__Rork Media/Scuba Diver", BASE_PATH),
	);

	// Royal Aeronautical Society
	destination_map.insert(
		"Aerospace 20".to_string(),
		format!("{}__Royal Aeronautical Society/Aerospace", BASE_PATH),
	);

	// RUSSH Media Pty
	destination_map.insert(
		"RUSSH ".to_string(),
		format!("{}__RUSSH Media Pty", BASE_PATH),
	);

	// SAE Media
	destination_map.insert(
		"ADAS & Autonomous Vehicle Engineering ".to_string(),
		format!("{}__SAE Media/ADAS & Autonomous Vehicle Engineering", BASE_PATH),
	);
	destination_map.insert(
		"Aerospace & Defense Technology ".to_string(),
		format!("{}__SAE Media/Aerospace & Defense Technology", BASE_PATH),
	);
	destination_map.insert(
		"Automotive Engineering ".to_string(),
		format!("{}__SAE Media/Automotive Engineering", BASE_PATH),
	);

	// Sainsbury
	destination_map.insert(
		"Sainsbury's Magazine ".to_string(),
		format!("{}__Sainsbury/Sainsbury's Magazine", BASE_PATH),
	);

	// Sandscreative LLC
	destination_map.insert(
		"Smore 20".to_string(),
		format!("{}__Sandscreative LLC/Smore", BASE_PATH),
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

	// SCG Magazine
	destination_map.insert(
		"Dish #".to_string(),
		format!("{}__SCG Magazine/Dish", BASE_PATH),
	);

	// Scranton Gillette Communications
	destination_map.insert(
		"Furniture Lighting & Decor ".to_string(),
		format!("{}__Scranton Gillette Communications/Furniture Lighting & Decor", BASE_PATH),
	);

	// Select Publisher Services
	destination_map.insert(
		"Writers' Forum ".to_string(),
		format!("{}__Select Publisher Services/Writers' Forum", BASE_PATH),
	);

	// Seymour
	destination_map.insert(
		"The Oldie ".to_string(),
		format!("{}__Seymour/The Oldie", BASE_PATH),
	);
	destination_map.insert(
		"Warships International Fleet Review ".to_string(),
		format!("{}__Seymour/Warships International Fleet Review", BASE_PATH),
	);

	// Showdetails
	destination_map.insert(
		"Showdetails ".to_string(),
		format!("{}__Showdetails/Showdetails", BASE_PATH),
	);

	// Sightline
	destination_map.insert(
		"Air Force Times ".to_string(),
		format!("{}__Sightline/Air Force Times", BASE_PATH),
	);
	destination_map.insert(
		"Army Times ".to_string(),
		format!("{}__Sightline/Army Times", BASE_PATH),
	);
	destination_map.insert(
		"Defense News ".to_string(),
		format!("{}__Sightline/Defense News", BASE_PATH),
	);
	destination_map.insert(
		"Navy Times ".to_string(),
		format!("{}__Sightline/Navy Times", BASE_PATH),
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
		"Shindig! ".to_string(),
		format!("{}__Silverback Publishing/Shindig!", BASE_PATH),
	);

	// Simon Brew Limited
	destination_map.insert(
		"Film Stories #".to_string(),
		format!("{}__Simon Brew Limited/Film Stories", BASE_PATH),
	);

	// Sleeper Media
	destination_map.insert(
		"Sleeper #".to_string(),
		format!("{}__Sleeper Media/Sleeper", BASE_PATH),
	);
	destination_map.insert(
		"Supper #".to_string(),
		format!("{}__Sleeper Media/Supper", BASE_PATH),
	);

	// Smithsonian Institution
	destination_map.insert(
		"Smithsonian ".to_string(),
		format!("{}__Smithsonian Institution/Smithsonian", BASE_PATH),
	);

	// Society for Science
	destination_map.insert(
		"Science News 20".to_string(),
		format!("{}__Society for Science/Science News", BASE_PATH),
	);
	destination_map.insert(
		"Science News Explores ".to_string(),
		format!("{}__Society for Science/Science News Explores", BASE_PATH),
	);

	// Sola Group
	destination_map.insert(
		"Kitchen & Bath Design News ".to_string(),
		format!("{}__Sola Group/Kitchen & Bath Design News", BASE_PATH),
	);
	destination_map.insert(
		"Qualified Remodeler ".to_string(),
		format!("{}__Sola Group/Qualified Remodeler", BASE_PATH),
	);
	destination_map.insert(
		"Residential Design ".to_string(),
		format!("{}__Sola Group/Residential Design", BASE_PATH),
	);

	// SOS Publications Group
	destination_map.insert(
		"Sound On Sound UK ".to_string(),
		format!("{}__SOS Publications Group/Sound On Sound UK", BASE_PATH),
	);
	destination_map.insert(
		"Sound On Sound US ".to_string(),
		format!("{}__SOS Publications Group/Sound On Sound US", BASE_PATH),
	);

	// Source2Create
	destination_map.insert(
		"Women In Security Magazine ".to_string(),
		format!("{}__Source2Create/Women In Security Magazine", BASE_PATH),
	);

	// Sovereign Media
	destination_map.insert(
		"Military Heritage ".to_string(),
		format!("{}__Sovereign Media/Military Heritage", BASE_PATH),
	);
	destination_map.insert(
		"WWII History ".to_string(),
		format!("{}__Sovereign Media/WWII History", BASE_PATH),
	);
	destination_map.insert(
		"WWII Quarterly ".to_string(),
		format!("{}__Sovereign Media/WWII Quarterly", BASE_PATH),
	);

	// Sparkle Buds
	destination_map.insert(
		"3D #".to_string(),
		format!("{}__Sparkle Buds/3D", BASE_PATH),
	);
	destination_map.insert(
		"ADHDEFG ".to_string(),
		format!("{}__Sparkle Buds/ADHDEFG", BASE_PATH),
	);
	destination_map.insert(
		"Brain Train ".to_string(),
		format!("{}__Sparkle Buds/Brain Train", BASE_PATH),
	);
	destination_map.insert(
		"De-Stress Yourself ".to_string(),
		format!("{}__Sparkle Buds/De-Stress Yourself", BASE_PATH),
	);
	destination_map.insert(
		"Foughtism ".to_string(),
		format!("{}__Sparkle Buds/Foughtism", BASE_PATH),
	);
	destination_map.insert(
		"Going Places ".to_string(),
		format!("{}__Sparkle Buds/Going Places", BASE_PATH),
	);
	destination_map.insert(
		"Little Sparkles ".to_string(),
		format!("{}__Sparkle Buds/Little Sparkles", BASE_PATH),
	);
	destination_map.insert(
		"Magic Logic ".to_string(),
		format!("{}__Sparkle Buds/Magic Logic", BASE_PATH),
	);
	destination_map.insert(
		"Sparkle Buds ".to_string(),
		format!("{}__Sparkle Buds/Sparkle Buds", BASE_PATH),
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

	// Spielbox
	destination_map.insert(
		"Spielbox ".to_string(),
		format!("{}__Spielbox/Spielbox", BASE_PATH),
	);

	// Springer Nature America
	destination_map.insert(
		"Nature 20".to_string(),
		format!("{}__Springer Nature America/Nature", BASE_PATH),
	);
	destination_map.insert(
		"Scientific American ".to_string(),
		format!("{}__Springer Nature America/Scientific American", BASE_PATH),
	);

	// Squires Kitchen
	destination_map.insert(
		"Cakes & Sugarcraft ".to_string(),
		format!("{}__Squires Kitchen/Cakes & Sugarcraft", BASE_PATH),
	);

	// St. Joseph Communications
	destination_map.insert(
		"Canadian Business ".to_string(),
		format!("{}__St. Joseph Communications/Canadian Business", BASE_PATH),
	);
	destination_map.insert(
		"Chatelaine ".to_string(),
		format!("{}__St. Joseph Communications/Chatelaine", BASE_PATH),
	);

	// Standard Publications
	destination_map.insert(
		"Taste & Flair ".to_string(),
		format!("{}__Standard Publications/Taste & Flair", BASE_PATH),
	);

	// Stella Novus
	destination_map.insert(
		"Ancient Origins Magazine ".to_string(),
		format!("{}__Stella Novus/Ancient Origins Magazine", BASE_PATH),
	);

	// Storytime Magazine
	destination_map.insert(
		"Storytime ".to_string(),
		format!("{}___Storytime Magazine/Storytime", BASE_PATH),
	);

	// Strategy & Tactics Press
	destination_map.insert(
		"World at War ".to_string(),
		format!("{}__Strategy & Tactics Press/World at War", BASE_PATH),
	);

	// Streamline Publishing
	destination_map.insert(
		"Fine Art Connoisseur ".to_string(),
		format!("{}__Streamline Publishing/Fine Art Connoisseur", BASE_PATH),
	);
	destination_map.insert(
		"Plein Air Magazine ".to_string(),
		format!("{}__Streamline Publishing/Plein Air Magazine", BASE_PATH),
	);

	// Summit Media Inc
	destination_map.insert(
		"Cosmopolitan Home ".to_string(),
		format!("{}__Summit Media Inc/Cosmopolitan Home", BASE_PATH),
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
	destination_map.insert(
		"Creative Artist ".to_string(),
		format!("{}__Sunray Publications/Creative Artist", BASE_PATH),
	);
	destination_map.insert(
		"Creative Beading ".to_string(),
		format!("{}__Sunray Publications/Creative Beading", BASE_PATH),
	);
	destination_map.insert(
		"Creative Sugarcraft ".to_string(),
		format!("{}__Sunray Publications/Creative Sugarcraft", BASE_PATH),
	);
	destination_map.insert(
		"Home Grown ".to_string(),
		format!("{}__Sunray Publications/Home Grown", BASE_PATH),
	);

	// Syon Media
	destination_map.insert(
		"Geographical ".to_string(),
		format!("{}__Syon Media/Geographical", BASE_PATH),
	);

	// T & L Publications
	destination_map.insert(
		"Nuts and Volts ".to_string(),
		format!("{}__T & L Publications/Nuts and Volts", BASE_PATH),
	);
	destination_map.insert(
		"Servo Magazine ".to_string(),
		format!("{}__T & L Publications/Servo Magazine", BASE_PATH),
	);

	// Tandy Media
	destination_map.insert(
		"Shipping Today & Yesterday ".to_string(),
		format!("{}__Tandy Media/Shipping Today & Yesterday", BASE_PATH),
	);

	// Taylist Media
	destination_map.insert(
		"Kitchens Bedrooms & Bathrooms ".to_string(),
		format!("{}__Taylist Media/Kitchens Bedrooms & Bathrooms", BASE_PATH),
	);

	// TCOLondon
	destination_map.insert(
		"Little White Lies ".to_string(),
		format!("{}__TCOLondon/Little White Lies", BASE_PATH),
	);

	// Tech Briefs Media
	destination_map.insert(
		"Aerospace & Defense Technology ".to_string(),
		format!("{}__Tech Briefs Media/Aerospace & Defense Technology", BASE_PATH),
	);

	// Techfastly
	destination_map.insert(
		"Techfastly ".to_string(),
		format!("{}__Techfastly/Techfastly", BASE_PATH),
	);

	// Telegraph Media Group
	destination_map.insert(
		"Apollo ".to_string(),
		format!("{}__Telegraph Media Group/Apollo", BASE_PATH),
	);
	destination_map.insert(
		"The Telegraph Magazine ".to_string(),
		format!("{}__Telegraph Media Group/The Telegraph Magazine", BASE_PATH),
	);

	// Television Academy
	destination_map.insert(
		"Emmy ".to_string(),
		format!("{}__Television Academy/Emmy", BASE_PATH),
	);

	// Temple Media
	destination_map.insert(
		"Passive House ".to_string(),
		format!("{}__Temple Media/Passive House", BASE_PATH),
	);

	// TEN Publishing
	destination_map.insert(
		"Recoil ".to_string(),
		format!("{}__TEN Publishing/Recoil", BASE_PATH),
	);

	// Texas Democracy Foundation
	destination_map.insert(
		"Texas Observer ".to_string(),
		format!("{}__Texas Democracy Foundation/Texas Observer", BASE_PATH),
	);

	// Texas Department of Transportation
	destination_map.insert(
		"Texas Highways ".to_string(),
		format!("{}__Texas Department of Transportation/Texas Highways", BASE_PATH),
	);

	// Texas Monthly
	destination_map.insert(
		"Texas Monthly ".to_string(),
		format!("{}__Texas Monthly/Texas Monthly", BASE_PATH),
	);

	// Texere Publishing
	destination_map.insert(
		"The Analytical Scientist ".to_string(),
		format!("{}__Texere Publishing/The Analytical Scientist", BASE_PATH),
	);

	// The Allée Group
	destination_map.insert(
		"Garden & Gun ".to_string(),
		format!("{}__The Allée Group/Garden & Gun", BASE_PATH),
	);

	// The American Society of Cinematographers
	destination_map.insert(
		"American Cinematographer ".to_string(),
		format!(
			"{}__The American Society of Cinematographers/American Cinematographer"
			, BASE_PATH
		),
	);

	// The Atlantic Monthly Group
	destination_map.insert(
		"The Atlantic ".to_string(),
		format!("{}__The Atlantic Monthly Group/The Atlantic", BASE_PATH),
	);

	// The Aviation Magazine
	destination_map.insert(
		"The Aviation Magazine ".to_string(),
		format!("{}__The Aviation Magazine/The Aviation Magazine", BASE_PATH),
	);

	// The Canadian Money Saver Inc
	destination_map.insert(
		"Canadian Money Saver ".to_string(),
		format!("{}__The Canadian Money Saver Inc/Canadian Money Saver", BASE_PATH),
	);

	// The Chap Ltd
	destination_map.insert(
		"Chap #".to_string(),
		format!("{}__The Chap Ltd/Chap", BASE_PATH),
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
		"Britain - ".to_string(),
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
		"Scotland #".to_string(),
		format!("{}__The Chelsea Magazine Company/Scotland", BASE_PATH),
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
	destination_map.insert(
		"The Economist ".to_string(),
		format!("{}__The Economist Newspaper/The Economist", BASE_PATH),
	);

	// The Explorers Club
	destination_map.insert(
		"The Explorers Journal ".to_string(),
		format!("{}__The Explorers Club/The Explorers Journal", BASE_PATH),
	);

	// The Illustrated Press
	destination_map.insert(
		"Illustration #".to_string(),
		format!("{}__The Illustrated Press/Illustration", BASE_PATH),
	);

	// The International Art Market Magazine
	destination_map.insert(
		"Art Market ".to_string(),
		format!("{}__The International Art Market Magazine/Art Market", BASE_PATH),
	);

	// The International Yachting Media
	destination_map.insert(
		"Yacht Digest ".to_string(),
		format!("{}__The International Yachting Media/Yacht Digest", BASE_PATH),
	);

	// The Nation Company
	destination_map.insert(
		"The Nation ".to_string(),
		format!("{}__The Nation Company/The Nation", BASE_PATH),
	);

	// The National Association for Amateur Radio
	destination_map.insert(
		"QST Magazine ".to_string(),
		format!("{}__The National Association for Amateur Radio/QST Magazine", BASE_PATH),
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

	// The Oxford American Literary Project
	destination_map.insert(
		"Oxford American ".to_string(),
		format!("{}__The Oxford American Literary Project/Oxford American", BASE_PATH),
	);

	// The Pluto Group
	destination_map.insert(
		"Homestyle NZ ".to_string(),
		format!("{}__The Pluto Group/Homestyle NZ", BASE_PATH),
	);

	// The Skeptics Society
	destination_map.insert(
		"Skeptic #".to_string(),
		format!("{}__The Skeptics Society/Skeptic", BASE_PATH),
	);

	// The Stylist Group
	destination_map.insert(
		"Stylist UK ".to_string(),
		format!("{}__The Stylist Group/Stylist UK", BASE_PATH),
	);

	// The Times Literary Supplement
	destination_map.insert(
		"The Times Literary Supplement ".to_string(),
		format!("{}__The Times Literary Supplement/The Times Literary Supplement", BASE_PATH),
	);

	// The Walrus Foundation
	destination_map.insert(
		"The Walrus ".to_string(),
		format!("{}__The Walrus Foundation/The Walrus", BASE_PATH),
	);

	// The Washington Post
	destination_map.insert(
		"The Washington Post Magazine ".to_string(),
		format!("{}__The Washington Post/The Washington Post Magazine", BASE_PATH),
	);

	// Theatre Communications Group
	destination_map.insert(
		"American Theatre ".to_string(),
		format!("{}__Theatre Communications Group/American Theatre", BASE_PATH),
	);

	// This Old House Ventures
	destination_map.insert(
		"This Old House ".to_string(),
		format!("{}__This Old House Ventures/This Old House", BASE_PATH),
	);

	// Time Magazine UK
	destination_map.insert(
		"Time EU ".to_string(),
		format!("{}__Time Magazine UK/Time EU", BASE_PATH),
	);
	destination_map.insert(
		"Time International ".to_string(),
		format!("{}__Time Magazine UK/Time International", BASE_PATH),
	);

	// Time USA
	destination_map.insert(
		"Time 20".to_string(),
		format!("{}__Time USA/Time", BASE_PATH),
	);
	destination_map.insert(
		"Time -".to_string(),
		format!("{}__Time USA/Time", BASE_PATH),
	);
	destination_map.insert(
		"Time for Kids".to_string(),
		format!("{}__Time USA/Time", BASE_PATH),
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

	// Tip Berlin Media Group
	destination_map.insert(
		"Exberliner ".to_string(),
		format!("{}__Tip Berlin Media Group/Exberliner", BASE_PATH),
	);

	// Token Publishing
	destination_map.insert(
		"Coin News ".to_string(),
		format!("{}__Token Publishing/Coin News", BASE_PATH),
	);
	destination_map.insert(
		"Medal News ".to_string(),
		format!("{}__Token Publishing/Medal News", BASE_PATH),
	);

	// Toronto Life Publishing
	destination_map.insert(
		"Toronto Life ".to_string(),
		format!("{}__Toronto Life Publishing/Toronto Life", BASE_PATH),
	);

	// Towse Publishing Co
	destination_map.insert(
		"Furniture World ".to_string(),
		format!("{}__Towse Publishing Co/Furniture World", BASE_PATH),
	);

	// Tribune Publishing Company
	destination_map.insert(
		"Chicago 20".to_string(),
		format!("{}__Tribune Publishing Company/Chicago", BASE_PATH),
	);

	// True North Media
	destination_map.insert(
		"Coffee Magazine ".to_string(),
		format!("{}__True North Media/Coffee Magazine", BASE_PATH),
	);

	// True West Publishing
	destination_map.insert(
		"True West ".to_string(),
		format!("{}__True West Publishing/True West", BASE_PATH),
	);

	// Trusted Media Brands
	destination_map.insert(
		"Birds & Blooms ".to_string(),
		format!("{}__Trusted Media Brands/Birds & Blooms", BASE_PATH),
	);
	destination_map.insert(
		"Country Woman ".to_string(),
		format!("{}__Trusted Media Brands/Country Woman", BASE_PATH),
	);
	destination_map.insert(
		"Reader's Digest US ".to_string(),
		format!("{}__Trusted Media Brands/Reader's Digest US", BASE_PATH),
	);
	destination_map.insert(
		"Taste of Home ".to_string(),
		format!("{}__Trusted Media Brands/Taste of Home", BASE_PATH),
	);
	destination_map.insert(
		"Family Handyman ".to_string(),
		format!("{}__Trusted Media Brands/Family Handyman", BASE_PATH),
	);

	// TVA
	destination_map.insert(
		"Canadian Living ".to_string(),
		format!("{}__TVA/Canadian Living", BASE_PATH),
	);
	destination_map.insert(
		"Style at Home CA ".to_string(),
		format!("{}__TVA/Style at Home CA", BASE_PATH),
	);

	// UK Rock & Roll
	destination_map.insert(
		"UK Rock & Roll ".to_string(),
		format!("{}__UK Rock & Roll/UK Rock & Roll", BASE_PATH),
	);

	// Ulster Journals Ltd
	destination_map.insert(
		"Perspective #".to_string(),
		format!("{}__Ulster Journals Ltd/Perspective", BASE_PATH),
	);

	// Ulster Tatler Group
	destination_map.insert(
		"Ulster Tatler 20".to_string(),
		format!("{}__Ulster Tatler Group/Ulster Tatler", BASE_PATH),
	);
	destination_map.insert(
		"Ulster Tatler Interiors ".to_string(),
		format!("{}__Ulster Tatler Group/Ulster Tatler Interiors", BASE_PATH),
	);

	// Uncooked Media
	destination_map.insert(
		"NEO Magazine ".to_string(),
		format!("{}__Uncooked Media/NEO Magazine", BASE_PATH),
	);

	// Under the Radar
	destination_map.insert(
		"Under the Radar ".to_string(),
		format!("{}__Under the Radar/Under the Radar", BASE_PATH),
	);

	// Univ. of Toronto
	destination_map.insert(
		"Rotman Management ".to_string(),
		format!("{}__Univ. of Toronto/Rotman Management", BASE_PATH),
	);

	// United Media Group
	destination_map.insert(
		"Melbourne Home Design + Living ".to_string(),
		format!("{}__United Media Group/Melbourne Home Design + Living", BASE_PATH),
	);
	destination_map.insert(
		"Melbourne Kitchen + Bathroom Design ".to_string(),
		format!("{}__United Media Group/Melbourne Kitchen + Bathroom Design", BASE_PATH),
	);
	destination_map.insert(
		"Melbourne Pool + Outdoor Design ".to_string(),
		format!("{}__United Media Group/Melbourne Pool + Outdoor Design", BASE_PATH),
	);

	// Universal Media Co.
	destination_map.insert(
		"Australian Country ".to_string(),
		format!("{}__Universal Media Co./Australian Country", BASE_PATH),
	);
	destination_map.insert(
		"Backyard & Outdoor Living ".to_string(),
		format!("{}__Universal Media Co./Backyard & Outdoor Living", BASE_PATH),
	);
	destination_map.insert(
		"EatWell ".to_string(),
		format!("{}__Universal Media Co./EatWell", BASE_PATH),
	);
	destination_map.insert(
		"Good Organic Gardening ".to_string(),
		format!("{}__Universal Media Co./Good Organic Gardening", BASE_PATH),
	);
	destination_map.insert(
		"Home Design ".to_string(),
		format!("{}__Universal Media Co./Home Design", BASE_PATH),
	);
	destination_map.insert(
		"Kitchen Yearbook ".to_string(),
		format!("{}__Universal Media Co./Kitchen Yearbook", BASE_PATH),
	);
	destination_map.insert(
		"Kitchens & Bathrooms Quarterly ".to_string(),
		format!("{}__Universal Media Co./Kitchens & Bathrooms Quarterly", BASE_PATH),
	);
	destination_map.insert(
		"Outdoor Design ".to_string(),
		format!("{}__Universal Media Co./Outdoor Design", BASE_PATH),
	);

	// Unqiue Homes Media
	destination_map.insert(
		"Unique Homes ".to_string(),
		format!("{}__Unqiue Homes Media/Unique Homes", BASE_PATH),
	);

	// USFRSC
	destination_map.insert(
		"Cowboys & Indians ".to_string(),
		format!("{}__USFRSC/Cowboys & Indians", BASE_PATH),
	);

	// VegOut Media
	destination_map.insert(
		"VegOut ".to_string(),
		format!("{}__VegOut Media/VegOut", BASE_PATH),
	);

	// Vintage Guitar
	destination_map.insert(
		"Vintage Guitar Magazine ".to_string(),
		format!("{}__Vintage Guitar/Vintage Guitar Magazine", BASE_PATH),
	);

	// Vivat Direct
	destination_map.insert(
		"Reader's Digest UK ".to_string(),
		format!("{}__Vivat Direct/Reader's Digest UK", BASE_PATH),
	);

	// Vox Media
	destination_map.insert(
		"New York Magazine ".to_string(),
		format!("{}__Vox Media/New York Magazine", BASE_PATH),
	);

	// Waddell
	destination_map.insert(
		"Dazed 20".to_string(),
		format!("{}__Waddell/Dazed", BASE_PATH),
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
		"Pianist #".to_string(),
		format!("{}__Warners/Pianist", BASE_PATH),
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

	// Wavelengths 10
	destination_map.insert(
		"Future Flight ".to_string(),
		format!("{}__Wavelengths 10/Future Flight", BASE_PATH),
	);

	// WDDTY Publishing
	destination_map.insert(
		"What Doctors Don't Tell You ".to_string(),
		format!("{}__WDDTY Publishing/What Doctors Don't Tell You", BASE_PATH),
	);

	// Western Business Media
	destination_map.insert(
		"Professional Security Installer ".to_string(),
		format!("{}__Western Business Media/Professional Security Installer", BASE_PATH),
	);

	// Westwick-Farrow Media
	destination_map.insert(
		"ECD 20".to_string(),
		format!("{}__Westwick-Farrow Media/ECD", BASE_PATH),
	);
	destination_map.insert(
		"Lab+Life Scientist ".to_string(),
		format!("{}__Westwick-Farrow Media/Lab+Life Scientist", BASE_PATH),
	);
	destination_map.insert(
		"What's New in Electronics ".to_string(),
		format!("{}__Westwick-Farrow Media/What's New in Electronics", BASE_PATH),
	);
	destination_map.insert(
		"What's New in Food Technology & Manufacturing ".to_string(),
		format!(
			"{}__Westwick-Farrow Media/What's New in Food Technology & Manufacturing", BASE_PATH
		),
	);

	// What on Earth Magazines
	destination_map.insert(
		"What on Earth! Magazine ".to_string(),
		format!("{}__What on Earth Magazines/What on Earth! Magazine", BASE_PATH),
	);

	// William Gibbons & Sons
	destination_map.insert(
		"Good Homes UK ".to_string(),
		format!("{}__William Gibbons & Sons/Good Homes UK", BASE_PATH),
	);

	// WoodenBoat Publications
	destination_map.insert(
		"Professional BoatBuilder ".to_string(),
		format!("{}__WoodenBoat Publications/Professional BoatBuilder", BASE_PATH),
	);
	destination_map.insert(
		"WoodenBoat ".to_string(),
		format!("{}__WoodenBoat Publications/WoodenBoat", BASE_PATH),
	);

	// Woodlands Publishing
	destination_map.insert(
		"Artist's Back to Basics ".to_string(),
		format!("{}__Woodlands Publishing/Artist's Back to Basics", BASE_PATH),
	);

	// World Poetry
	destination_map.insert(
		"The American Poetry Review ".to_string(),
		format!("{}__World Poetry/The American Poetry Review", BASE_PATH),
	);

	// WTWH Media
	destination_map.insert(
		"Design World ".to_string(),
		format!("{}__WTWH Media/Design World", BASE_PATH),
	);
	destination_map.insert(
		"EEWorld ".to_string(),
		format!("{}__WTWH Media/EEWorld", BASE_PATH),
	);
	destination_map.insert(
		"Fluid Power World ".to_string(),
		format!("{}__WTWH Media/Fluid Power World", BASE_PATH),
	);

	// X3DMedia
	destination_map.insert(
		"DEVELOP3D ".to_string(),
		format!("{}__X3DMedia/DEVELOP3D", BASE_PATH),
	);

	// Yaffa Media
	destination_map.insert(
		"Great Walks AU ".to_string(),
		format!("{}__Yaffa Media/Great Walks AU", BASE_PATH),
	);

	// Yankee Publishing
	destination_map.insert(
		"Family Tree US ".to_string(),
		format!("{}__Yankee Publishing/Family Tree US", BASE_PATH),
	);

	// Yoga Magazine
	destination_map.insert(
		"Yoga Magazine ".to_string(),
		format!("{}__Yoga Magazine/Yoga Magazine", BASE_PATH),
	);

	// Zahra Publishing
	destination_map.insert(
		"easyFood ".to_string(),
		format!("{}__Zahra Publishing/easyFood", BASE_PATH),
	);
	destination_map.insert(
		"Every Cook ".to_string(),
		format!("{}__Zahra Publishing/Every Cook", BASE_PATH),
	);

	// Zeit
	destination_map.insert(
		"Business Spotlight ".to_string(),
		format!("{}__Zeit/Business Spotlight", BASE_PATH),
	);
	destination_map.insert(
		"Spotlight 20".to_string(),
		format!("{}__Zeit/Spotlight", BASE_PATH),
	);

	// Zest Media
	destination_map.insert(
		"Absolutely Education ".to_string(),
		format!("{}__Zest Media/Absolutely Education", BASE_PATH),
	);

	// Ziff Davis Inc
	destination_map.insert(
		"PC Magazine 20".to_string(),
		format!("{}__Ziff Davis Inc/PC Magazine", BASE_PATH),
	);

	// Zonda Media
	destination_map.insert(
		"Architect 20".to_string(),
		format!("{}__Zonda Media/Architect", BASE_PATH),
	);
	destination_map.insert(
		"Builder 20".to_string(),
		format!("{}__Zonda Media/Builder", BASE_PATH),
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
