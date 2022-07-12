use std::{thread, time};
use rand::thread_rng;
use rand::seq::SliceRandom;

const C_BOMB: &'static [&'static str] = &["3333","4444","5555","6666","7777","8888","9999","TTTT","JJJJ","QQQQ","KKKK","AAAA","2222","BR"];
const C_S:    &'static [&'static str] = &["3","4","5","6","7","8","9","T","J","Q","K","A","2","B","R"];
const C_D:    &'static [&'static str] = &["33","44","55","66","77","88","99","TT","JJ","QQ","KK","AA","22"];
const C_T:    &'static [&'static str] = &["333","444","555","666","777","888","999","TTT","JJJ","QQQ","KKK","AAA","222"];
const C_Q5:   &'static [&'static str] = &["34567","45678","56789","6789T","789TJ","89TJQ","9TJQK","TJQKA"];
const C_Q6:   &'static [&'static str] = &["345678","456789","56789T","6789TJ","789TJQ","89TJQK","9TJQKA"];
const C_Q7:   &'static [&'static str] = &["3456789","456789T","56789TJ","6789TJQ","789TJQK","89TJQKA"];
const C_Q8:   &'static [&'static str] = &["3456789T","456789TJ","56789TJQ","6789TJQK","789TJQKA"];
const C_Q9:   &'static [&'static str] = &["3456789TJ","456789TJQ","56789TJQK","6789TJQKA"];
const C_Q10:  &'static [&'static str] = &["3456789TJQ","456789TJQK","56789TJQKA"];
const C_Q11:  &'static [&'static str] = &["3456789TJQK","456789TJQKA"];
const C_Q12:  &'static [&'static str] = &["3456789TJQKA"];
const C_2Q3:  &'static [&'static str] = &["334455","445566","556677","667788","778899","8899TT","99TTJJ","TTJJQQ","JJQQKK","QQKKAA"];
const C_2Q4:  &'static [&'static str] = &["33445566","44556677","55667788","66778899","778899TT","8899TTJJ","99TTJJQQ","TTJJQQKK","JJQQKKAA"];
const C_2Q5:  &'static [&'static str] = &["3344556677","4455667788","5566778899","66778899TT","778899TTJJ","8899TTJJQQ","99TTJJQQKK","TTJJQQKKAA"];
const C_2Q6:  &'static [&'static str] = &["334455667788","445566778899","5566778899TT","66778899TTJJ","778899TTJJQQ","8899TTJJQQKK","99TTJJQQKKAA"];
const C_2Q7:  &'static [&'static str] = &["33445566778899","445566778899TT","5566778899TTJJ","66778899TTJJQQ","778899TTJJQQKK","8899TTJJQQKKAA"];
const C_2Q8:  &'static [&'static str] = &["33445566778899TT","445566778899TTJJ","5566778899TTJJQQ","66778899TTJJQQKK","778899TTJJQQKKAA"];
const C_2Q9:  &'static [&'static str] = &["33445566778899TTJJ","445566778899TTJJQQ","5566778899TTJJQQKK","66778899TTJJQQKKAA"];
const C_2Q10: &'static [&'static str] = &["33445566778899TTJJQQ","445566778899TTJJQQKK","5566778899TTJJQQKKAA"];
const C_3Q2:  &'static [&'static str] = &["333444","444555","555666","666777","777888","888999","999TTT","TTTJJJ","JJJQQQ","QQQKKK","KKKAAA"];
const C_3Q3:  &'static [&'static str] = &["333444555","444555666","555666777","666777888","777888999","888999TTT","999TTTJJJ","TTTJJJQQQ","JJJQQQKKK","QQQKKKAAA"];
const C_3Q4:  &'static [&'static str] = &["333444555666","444555666777","555666777888","666777888999","777888999TTT","888999TTTJJJ","999TTTJJJQQQ","TTTJJJQQQKKK","JJJQQQKKKAAA"];
const C_3Q5:  &'static [&'static str] = &["333444555666777","444555666777888","555666777888999","666777888999TTT","777888999TTTJJJ","888999TTTJJJQQQ","999TTTJJJQQQKKK","TTTJJJQQQKKKAAA"];
const C_3Q6:  &'static [&'static str] = &["333444555666777888","444555666777888999","555666777888999TTT","666777888999TTTJJJ","777888999TTTJJJQQQ","888999TTTJJJQQQKKK","999TTTJJJQQQKKKAAA"];

fn str_list_2_string_vec(vec: &mut Vec<String>, list: &[&str])
{
	for item in list
	{
		vec.push(item.to_string());
	}
}

struct DataBase
{
	c_bomb:   Vec<String>,
	c_s:      Vec<String>, // single
	c_d:      Vec<String>, // double
	c_t:      Vec<String>, // triple
	c_q5:     Vec<String>, // queue
	c_q6:     Vec<String>,
	c_q7:     Vec<String>,
	c_q8:     Vec<String>,
	c_q9:     Vec<String>,
	c_q10:    Vec<String>,
	c_q11:    Vec<String>,
	c_q12:    Vec<String>,
	c_2q3:    Vec<String>,
	c_2q4:    Vec<String>,
	c_2q5:    Vec<String>,
	c_2q6:    Vec<String>,
	c_2q7:    Vec<String>,
	c_2q8:    Vec<String>,
	c_2q9:    Vec<String>,
	c_2q10:   Vec<String>,
	c_3q2:    Vec<String>,
	c_3q3:    Vec<String>,
	c_3q4:    Vec<String>,
	c_3q5:    Vec<String>,
	c_3q6:    Vec<String>,
	
	c_t_s:    Vec<String>,
	c_t_d:    Vec<String>,
	c_3q2_s:  Vec<String>,
	c_3q2_d:  Vec<String>,
	c_3q3_s:  Vec<String>,
	c_3q3_d:  Vec<String>,
	c_3q4_s:  Vec<String>,
	c_3q4_d:  Vec<String>,
	c_3q5_s:  Vec<String>,
	c_bomb_s: Vec<String>,
	c_bomb_d: Vec<String>,
	
	c_active: Vec<String>,
	c_list_1: Vec<Vec<String>>,
	c_list_2: Vec<Vec<String>>,
}
impl DataBase
{
	fn new() -> DataBase
	{
		let mut db = DataBase{	c_s:      Vec::new(),
								c_d:      Vec::new(),
								c_t:      Vec::new(),
								c_t_s:    Vec::new(),
								c_t_d:    Vec::new(),
								c_q5:     Vec::new(),
								c_q6:     Vec::new(),
								c_q7:     Vec::new(),
								c_q8:     Vec::new(),
								c_q9:     Vec::new(),
								c_q10:    Vec::new(),
								c_q11:    Vec::new(),
								c_q12:    Vec::new(),
								c_2q3:    Vec::new(),
								c_2q4:    Vec::new(),
								c_2q5:    Vec::new(),
								c_2q6:    Vec::new(),
								c_2q7:    Vec::new(),
								c_2q8:    Vec::new(),
								c_2q9:    Vec::new(),
								c_2q10:   Vec::new(),
								c_3q2:    Vec::new(),
								c_3q2_s:  Vec::new(),
								c_3q2_d:  Vec::new(),
								c_3q3:    Vec::new(),
								c_3q3_s:  Vec::new(),
								c_3q3_d:  Vec::new(),
								c_3q4:    Vec::new(),
								c_3q4_s:  Vec::new(),
								c_3q4_d:  Vec::new(),
								c_3q5:    Vec::new(),
								c_3q5_s:  Vec::new(),
								c_3q6:    Vec::new(),
								c_bomb:   Vec::new(),
								c_bomb_s: Vec::new(),
								c_bomb_d: Vec::new(),
								c_active: Vec::new(),
								c_list_1: Vec::new(),
								c_list_2: Vec::new(),};
								
		str_list_2_string_vec(&mut db.c_bomb, C_BOMB);
		str_list_2_string_vec(&mut db.c_s,    C_S);
		str_list_2_string_vec(&mut db.c_d,    C_D);
		str_list_2_string_vec(&mut db.c_t,    C_T);
		str_list_2_string_vec(&mut db.c_q5,   C_Q5);
		str_list_2_string_vec(&mut db.c_q6,   C_Q6);
		str_list_2_string_vec(&mut db.c_q7,   C_Q7);
		str_list_2_string_vec(&mut db.c_q8,   C_Q8);
		str_list_2_string_vec(&mut db.c_q9,   C_Q9);
		str_list_2_string_vec(&mut db.c_q10,  C_Q10);
		str_list_2_string_vec(&mut db.c_q11,  C_Q11);
		str_list_2_string_vec(&mut db.c_q12,  C_Q12);
		str_list_2_string_vec(&mut db.c_2q3,  C_2Q3);
		str_list_2_string_vec(&mut db.c_2q4,  C_2Q4);
		str_list_2_string_vec(&mut db.c_2q5,  C_2Q5);
		str_list_2_string_vec(&mut db.c_2q6,  C_2Q6);
		str_list_2_string_vec(&mut db.c_2q7,  C_2Q7);
		str_list_2_string_vec(&mut db.c_2q8,  C_2Q8);
		str_list_2_string_vec(&mut db.c_2q9,  C_2Q9);
		str_list_2_string_vec(&mut db.c_2q10, C_2Q10);
		str_list_2_string_vec(&mut db.c_3q2,  C_3Q2);
		str_list_2_string_vec(&mut db.c_3q3,  C_3Q3);
		str_list_2_string_vec(&mut db.c_3q4,  C_3Q4);
		str_list_2_string_vec(&mut db.c_3q5,  C_3Q5);
		str_list_2_string_vec(&mut db.c_3q6,  C_3Q6);

		for aaa in &db.c_t 
		{
			for b in &db.c_s
			{
				db.c_t_s.push(aaa.to_owned()+b);
			}
		}
		
		for aaa in &db.c_t 
		{
			for bb in &db.c_d
			{
				db.c_t_d.push(aaa.to_owned()+bb);
			}
		}
		
		for aaabbb in &db.c_3q2 
		{			
			for c in &db.c_s {
			for d in &db.c_s {
				if !c.eq(d) {
					db.c_3q2_s.push(aaabbb.to_owned() + c + d);
			}}}
		}
		
		for aaabbb in &db.c_3q2 
		{			
			for cc in &db.c_d {
			for dd in &db.c_d {
				if !cc.eq(dd) {
						db.c_3q2_d.push(aaabbb.to_owned() + cc + dd);
			}}}
		}
		
		for aaabbbccc in &db.c_3q3 
		{			
			for d in &db.c_s {
			for e in &db.c_s {
			for f in &db.c_s {
				if !d.eq(e) && !e.eq(f) && !d.eq(f) {
					db.c_3q3_s.push(aaabbbccc.to_owned() + d + e + f);
			}}}}
		}
		
		for aaabbbccc in &db.c_3q3 
		{			
			for dd in &db.c_d {
			for ee in &db.c_d {
			for ff in &db.c_d {
				if !dd.eq(ee) && !ee.eq(ff) && !dd.eq(ff) {
					db.c_3q3_d.push(aaabbbccc.to_owned() + dd + ee + ff);
			}}}}
		}
		
		for aaabbbcccddd in &db.c_3q4 
		{			
			for e in &db.c_s {
			for f in &db.c_s {
			for g in &db.c_s {
			for h in &db.c_s {
				if !e.eq(f) && !e.eq(g) && !e.eq(h) && !f.eq(g) && !f.eq(h) && !g.eq(h) {
					db.c_3q4_s.push(aaabbbcccddd.to_owned() + e + f + g + h);
			}}}}}
		}
		
		for aaabbbcccddd in &db.c_3q4 
		{			
			for ee in &db.c_d {
			for ff in &db.c_d {
			for gg in &db.c_d {
			for hh in &db.c_d {
				if !ee.eq(ff) && !ee.eq(gg) && !ee.eq(hh) && !ff.eq(gg) && !ff.eq(hh) && !gg.eq(hh) {
					db.c_3q4_d.push(aaabbbcccddd.to_owned() + ee + ff + gg + hh);
			}}}}}
		}
		
		for aaabbbcccdddeee in &db.c_3q5 
		{			
			for e in &db.c_s {
			for f in &db.c_s {
			for g in &db.c_s {
			for h in &db.c_s {
			for i in &db.c_s {
				if !e.eq(f) && !e.eq(g) && !e.eq(h) && !e.eq(i) && !f.eq(g) && !f.eq(h) && !f.eq(i) && !g.eq(h) && !g.eq(i) && !h.eq(i) {
					db.c_3q5_s.push(aaabbbcccdddeee.to_owned() + e + f + g + h + i);
			}}}}}}
		}
		
		for bomb in &db.c_bomb 
		{
			if !bomb.eq("BR")
			{
				for a in &db.c_s {
				for b in &db.c_s {
					if !a.eq(b) {
						db.c_bomb_s.push(bomb.to_owned() + a + b);
				}}}
			}
		}
		
		for bomb in &db.c_bomb 
		{
			if !bomb.eq("BR")
			{
				for aa in &db.c_d {
				for bb in &db.c_d {
					if !aa.eq(bb) {
						db.c_bomb_d.push(bomb.to_owned() + aa + bb);
				}}}
			}
		}
			
		db.c_list_2.push(db.c_3q5_s.to_owned());
		db.c_list_2.push(db.c_3q4_d.to_owned());
		db.c_list_2.push(db.c_2q10.to_owned());
		db.c_list_2.push(db.c_2q9.to_owned());
		db.c_list_2.push(db.c_3q6.to_owned());
		db.c_list_2.push(db.c_2q8.to_owned());
		db.c_list_2.push(db.c_3q4_s.to_owned());
		db.c_list_2.push(db.c_3q3_d.to_owned());
		db.c_list_2.push(db.c_3q5.to_owned());
		db.c_list_2.push(db.c_2q7.to_owned());
		db.c_list_2.push(db.c_3q3_s.to_owned());
		db.c_list_2.push(db.c_2q6.to_owned());
		db.c_list_2.push(db.c_3q4.to_owned());
		db.c_list_2.push(db.c_q12.to_owned());
		db.c_list_2.push(db.c_q11.to_owned());
		db.c_list_2.push(db.c_q10.to_owned());
		db.c_list_2.push(db.c_2q5.to_owned());
		db.c_list_2.push(db.c_3q2_d.to_owned());
		db.c_list_2.push(db.c_q9.to_owned());
		db.c_list_2.push(db.c_3q3.to_owned());
		db.c_list_2.push(db.c_2q4.to_owned());
		db.c_list_2.push(db.c_3q2_s.to_owned());
		db.c_list_2.push(db.c_q8.to_owned());
		db.c_list_2.push(db.c_q7.to_owned());
		db.c_list_2.push(db.c_2q3.to_owned());
		db.c_list_2.push(db.c_3q2.to_owned());
		db.c_list_2.push(db.c_q6.to_owned());	
		db.c_list_2.push(db.c_q5.to_owned());
		db.c_list_2.push(db.c_t_d.to_owned());
		db.c_list_2.push(db.c_t_s.to_owned());
		db.c_list_2.push(db.c_t.to_owned());
		db.c_list_2.push(db.c_d.to_owned());
		db.c_list_2.push(db.c_s.to_owned());
		db.c_list_2.push(db.c_bomb_d.to_owned());
		db.c_list_2.push(db.c_bomb_s.to_owned());	
		db.c_list_2.push(db.c_bomb.to_owned());
		
		for bomb in &db.c_bomb
		{
			db.c_s.push(bomb.to_owned());
			db.c_d.push(bomb.to_owned());
			db.c_t.push(bomb.to_owned());
			db.c_q5.push(bomb.to_owned());
			db.c_q6.push(bomb.to_owned());
			db.c_q7.push(bomb.to_owned());
			db.c_q8.push(bomb.to_owned());
			db.c_q9.push(bomb.to_owned());
			db.c_q10.push(bomb.to_owned());
			db.c_q11.push(bomb.to_owned());
			db.c_q12.push(bomb.to_owned());
			db.c_2q3.push(bomb.to_owned());
			db.c_2q4.push(bomb.to_owned());
			db.c_2q5.push(bomb.to_owned());
			db.c_2q6.push(bomb.to_owned());
			db.c_2q7.push(bomb.to_owned());
			db.c_2q8.push(bomb.to_owned());
			db.c_2q9.push(bomb.to_owned());
			db.c_2q10.push(bomb.to_owned());
			db.c_3q2.push(bomb.to_owned());
			db.c_3q3.push(bomb.to_owned());
			db.c_3q4.push(bomb.to_owned());
			db.c_3q5.push(bomb.to_owned());
			db.c_3q6.push(bomb.to_owned());
			db.c_t_s.push(bomb.to_owned());
			db.c_t_d.push(bomb.to_owned());
			db.c_3q2_s.push(bomb.to_owned());
			db.c_3q2_d.push(bomb.to_owned());
			db.c_3q3_s.push(bomb.to_owned());
			db.c_3q3_d.push(bomb.to_owned());
			db.c_3q4_s.push(bomb.to_owned());
			db.c_3q4_d.push(bomb.to_owned());
			db.c_3q5_s.push(bomb.to_owned());
			db.c_bomb_s.push(bomb.to_owned());
			db.c_bomb_d.push(bomb.to_owned());
		}
					
		db.c_list_1.push(db.c_s.to_owned());
		db.c_list_1.push(db.c_d.to_owned());
		db.c_list_1.push(db.c_t.to_owned());
		db.c_list_1.push(db.c_q5.to_owned());
		db.c_list_1.push(db.c_q6.to_owned());
		db.c_list_1.push(db.c_q7.to_owned());
		db.c_list_1.push(db.c_q8.to_owned());
		db.c_list_1.push(db.c_q9.to_owned());
		db.c_list_1.push(db.c_q10.to_owned());
		db.c_list_1.push(db.c_q11.to_owned());
		db.c_list_1.push(db.c_q12.to_owned());
		db.c_list_1.push(db.c_2q3.to_owned());
		db.c_list_1.push(db.c_2q4.to_owned());
		db.c_list_1.push(db.c_2q5.to_owned());
		db.c_list_1.push(db.c_2q6.to_owned());
		db.c_list_1.push(db.c_2q7.to_owned());
		db.c_list_1.push(db.c_2q8.to_owned());
		db.c_list_1.push(db.c_2q9.to_owned());
		db.c_list_1.push(db.c_2q10.to_owned());
		db.c_list_1.push(db.c_3q2.to_owned());
		db.c_list_1.push(db.c_3q3.to_owned());
		db.c_list_1.push(db.c_3q4.to_owned());
		db.c_list_1.push(db.c_3q5.to_owned());
		db.c_list_1.push(db.c_3q6.to_owned());
		db.c_list_1.push(db.c_t_s.to_owned());
		db.c_list_1.push(db.c_t_d.to_owned());
		db.c_list_1.push(db.c_3q2_s.to_owned());
		db.c_list_1.push(db.c_3q2_d.to_owned());
		db.c_list_1.push(db.c_3q3_s.to_owned());
		db.c_list_1.push(db.c_3q3_d.to_owned());
		db.c_list_1.push(db.c_3q4_s.to_owned());
		db.c_list_1.push(db.c_3q4_d.to_owned());
		db.c_list_1.push(db.c_3q5_s.to_owned());
		db.c_list_1.push(db.c_bomb_s.to_owned());		
		db.c_list_1.push(db.c_bomb_d.to_owned());
					
		return db;
	}
	
	fn set_active_combo(&mut self, cards: &Cards)
	{
		for combo_list in &self.c_list_1
		{
			for combo in combo_list
			{
				let mut chars: Vec<char> = combo.chars().collect();
					
				let mut names = Vec::new();
					
				for c in chars 
				{				
					let mut name = String::new();
					if c.eq(&'T') 
					{
						name.push_str("10");
					} 
					else 
					{
						name.push(c);
					}
						
					names.push(name);
				}
				
				if cards.equal_cards_by_names(names)
				{
					self.c_active = combo_list.to_owned();
					return;
				}
			}
		}
	}
	
	fn find_propriate_cards(&self, cards_in_hand: &Cards) -> Cards
	{
		for combo_list in &self.c_list_2
		{
			for combo in combo_list
			{
				if combo.len() > cards_in_hand.number()
				{
					continue
				}
				let mut chars: Vec<char> = combo.chars().collect();
					
				let mut names = Vec::new();
					
				for c in chars 
				{				
					let mut name = String::new();
					if c.eq(&'T') 
					{
						name.push_str("10");
					} 
					else 
					{
						name.push(c);
					}
						
					names.push(name);
				}
				
				let propriate_cards = cards_in_hand.find_cards_by_names(names);
				if propriate_cards.number() > 0
				{
					return propriate_cards;
				}
			}
		}

		Cards::new()
	}

	fn find_bigger_cards(&self, cards_to_beat: &Cards, cards_in_hand: &Cards) -> Cards
	{
		let mut cards_names_to_beat = String::new();
	
		for combo in &self.c_active
		{
			let mut chars: Vec<char> = combo.chars().collect();
					
			let mut names = Vec::new();
					
			for c in chars 
			{				
				let mut name = String::new();
				if c.eq(&'T') 
				{
					name.push_str("10");
				} 
				else 
				{
					name.push(c);
				}
						
				names.push(name);
			}
				
			if cards_names_to_beat.eq("")
			{
				if cards_to_beat.equal_cards_by_names(names)
				{
					cards_names_to_beat = combo.to_owned();
				}
			}
			else
			{
				let bigger_cards = cards_in_hand.find_cards_by_names(names);
				if bigger_cards.number() > 0
				{
					return bigger_cards;
				}
			}
		}
		
		Cards::new()
	}
}


fn main() 
{
	let mut db = DataBase::new();
	let mut deck = new_deck();
	
	let mut lord = deck.tail_pop_multi(20);
	let mut mary = deck.tail_pop_multi(17);
	let mut john = deck.tail_pop_multi(17);
	
	lord.set_owner("lord");
	mary.set_owner("mary");
	john.set_owner("john");
	
	lord.sort();
	mary.sort();
	john.sort();
	
	let mut pass_times = 0;
	let mut curren_player = &mut lord;
	let mut last_play = Cards::new();
	
	loop 
	{
		print!("\x1B[2J"); // clear screen

		println!();println!();println!();println!();println!();
		
		if curren_player.owned_by("lord") {println!("Last Play:");last_play.print(); println!("Lord:"); curren_player.print();}
		if curren_player.owned_by("mary") {println!("Mary:");}
		if curren_player.owned_by("john") {println!("John:");}
		
		let mut played = Cards::new();
		if !curren_player.owned_by("lord")
		{
			if last_play.number() == 0
			{
				played = db.find_propriate_cards(&curren_player);
			}
			else
			{
				played = db.find_bigger_cards(&last_play, &curren_player);
			}

			played.print();
			curren_player.remove_multi(&played);
			sleep(1);
		}
		else
		{
			let played_ret = play_by(&mut curren_player);
			played = match played_ret
			{
				Some(valid_played) => valid_played,
				None               => {println!("Play By Rules, Dude!"); sleep(2); continue;}
			};
		}
		if played.number() == 0 
		{
			if last_play.number() == 0
			{
				println!("Play Some Shit, Dude!");	
				sleep(2); // sleep 2 sec
				continue;
			}
			
			println!("Pass!");			
			sleep(1); // sleep 2 sec
			
			pass_times = pass_times + 1;
			
			if pass_times == 2
			{
				pass_times = 0;
				last_play = Cards::new();
			}
		} 
		else 
		{
			if last_play.number() == 0
			{
				db.set_active_combo(&played);
			}
			pass_times=0;
			last_play = played;
		} 
		
		if curren_player.number() == 0 
		{
			if curren_player.owned_by("lord") {println!("Lord WIN!")}
			if curren_player.owned_by("mary") {println!("Mary WIN!")}
			if curren_player.owned_by("john") {println!("John WIN!")}
			break;
		} 
		
			 if curren_player.owned_by("lord") {curren_player = &mut mary;}
		else if curren_player.owned_by("mary") {curren_player = &mut john;}
		else if curren_player.owned_by("john") {curren_player = &mut lord;}
    }
}

fn sleep(seconds: u64)
{
	thread::sleep(time::Duration::from_millis(seconds*1000));
}

fn play_by(cards: &mut Cards) -> Option<Cards>
{
	let mut line = String::new();
	let input_bytes_num = std::io::stdin().read_line(&mut line).unwrap();
	if input_bytes_num > 1 
	{
		line = line.replace("10","T");
		let mut chars: Vec<char> = line.chars().collect();
			
		chars.remove(chars.len()-1);
			
		let mut names = Vec::new();
			
		for c in chars 
		{				
			let mut name = String::new();
			if c.eq(&'T') 
			{
				name.push_str("10");
			} 
			else 
			{
				name.push(c);
			}
			
			names.push(name);
		}
			
		let played_cards = cards.find_cards_by_names(names);
		if played_cards.number() == 0
		{
			return None; // invalid play
		}
			
		cards.remove_multi(&played_cards);
		
		return Some(played_cards);
	} 
	else 
	{	
		return Some(Cards::new());
	}
}

struct Cards
{
	cards: Vec<Card>,
	owner: String,
	class: u32,
	rank:  u32
}
impl Cards
{
	fn new() -> Cards
	{		
		return Cards{cards: Vec::new(), owner: "".to_string(), class: 0, rank: 0};
	}
	fn set_rank(&mut self)
	{
		
	}
	fn set_owner(&mut self, owner: &str)
	{
		self.owner = owner.to_string();
	}
	fn owned_by(&self, some_one: &str) -> bool
	{
		return self.owner.eq(some_one);
	}
	fn get_owner(&self) -> &String
	{
		return &self.owner;
	}
	fn vec(&self) -> &Vec<Card>
	{
		return &self.cards;
	}
	fn number(&self) -> usize
	{
		return self.cards.len();
	}
	fn sort(&mut self)
	{
		self.cards.sort_by(|a, b| a.rank.cmp(&b.rank));
	}
	fn shuffle(&mut self)
	{
		self.cards.shuffle(&mut thread_rng());
	}
	fn add(&mut self, card: Card)
	{
		self.cards.push(card);
	}
	fn remove(&mut self, card: &Card)
	{
		for i in 0..self.cards.len() 
		{
			if card.class().eq(self.cards[i].class()) && card.name().eq(self.cards[i].name()) 
			{
				self.cards.remove(i);
				break;
			}
		}
	}
	fn remove_multi(&mut self, cards: &Cards)
	{
		for card in &cards.cards
		{
			self.remove(&card);
		}
	}
	fn tail_pop_multi(&mut self, pop_num: usize) -> Cards
	{
		let mut cards = Cards::new();
		let inhand_num = self.number();
		let vec_cards = self.cards.split_off(inhand_num-pop_num);
		
		for card in vec_cards
		{
			cards.add(card);
		}
		
		cards
	}
	fn find_cards_by_names(&self, mut names:Vec<String>) -> Cards
	{
		let mut found = Cards::new();
		let names_len = names.len(); 
	
		for card in self.vec() 
		{
			for i in 0..names.len() 
			{
				if names[i].eq(card.name()) 
				{
					names.remove(i);
					found.add(card.clone());
					break;
				}
			}
		}
		
		if names_len == found.number()
		{
			found
		}
		else
		{
			Cards::new()
		}
	}
	fn equal_cards_by_names(&self, mut names:Vec<String>) -> bool
	{
		let mut found = Cards::new();
		let names_len = names.len(); 
		let cards_len = self.number(); 
	
		for card in self.vec() 
		{
			for i in 0..names.len() 
			{
				if names[i].eq(card.name()) 
				{
					names.remove(i);
					found.add(card.clone());
					break;
				}
			}
		}
		
		if names_len == found.number() && names_len == cards_len
		{
			true
		}
		else
		{
			false
		}
	}
	fn print(&self)
	{
		let cards = &self.cards;
	
		let cards_num = cards.len();

		if cards_num == 0
		{
			println!("Empty!");
			println!();
			return;
		}

		print!(" ");
		for _i in 0..cards_num 
		{
				print!("___");
		}
		println!("__");

		for card in cards 
		{
			print!("|{} ", card.class());
		}
		println!("   |");
		
		for card in cards 
		{
			if card.name().as_str().eq("10") 
			{
				print!("|{}", card.name());
			} 
			else 
			{
				print!("|{} ", card.name());
			}
		}
		println!("   |");
		
		for _i in 0..cards_num 
		{
			print!("|  ");
		}
		println!("   |");
		
		for _i in 0..cards_num 
		{
			print!("|  ");
		}
		println!("   |");
		
		for _i in 0..cards_num 
		{
			print!("|__");
		}
		println!("___|");
		println!();
	}
}

struct Card
{
	class: String,
	name:  String,
	rank:  u32 
}

impl Card 
{
	fn new(cls:&str, nm:&str, rk:u32) -> Card 
	{
		Card{class: cls.to_string(), name: nm.to_string(),  rank: rk}
	}
	fn clone(&self) -> Card 
	{
		Card{class: self.class().clone(), name: self.name().clone(), rank: self.rank()}
	}
	fn class(&self) -> &String 
	{
		&self.class
	}
	fn name(&self) -> &String 
	{
		&self.name
	}
	fn rank(&self) -> u32 
	{
		self.rank
	}
}

fn new_deck() -> Cards
{
	let mut deck = Cards::new();
	
	deck.add(Card::new("♠","3",1));
	deck.add(Card::new("♠","4",2));
	deck.add(Card::new("♠","5",3));
	deck.add(Card::new("♠","6",4));
	deck.add(Card::new("♠","7",5));
	deck.add(Card::new("♠","8",6));
	deck.add(Card::new("♠","9",7));
	deck.add(Card::new("♠","10",8));
	deck.add(Card::new("♠","J",9));
	deck.add(Card::new("♠","Q",10));
	deck.add(Card::new("♠","K",11));
	deck.add(Card::new("♠","A",12));
	deck.add(Card::new("♠","2",20));
	
	deck.add(Card::new("♥","3",1));
	deck.add(Card::new("♥","4",2));
	deck.add(Card::new("♥","5",3));
	deck.add(Card::new("♥","6",4));
	deck.add(Card::new("♥","7",5));
	deck.add(Card::new("♥","8",6));
	deck.add(Card::new("♥","9",7));
	deck.add(Card::new("♥","10",8));
	deck.add(Card::new("♥","J",9));
	deck.add(Card::new("♥","Q",10));
	deck.add(Card::new("♥","K",11));
	deck.add(Card::new("♥","A",12));
	deck.add(Card::new("♥","2",20));
	
	deck.add(Card::new("♣","3",1));
	deck.add(Card::new("♣","4",2));
	deck.add(Card::new("♣","5",3));
	deck.add(Card::new("♣","6",4));
	deck.add(Card::new("♣","7",5));
	deck.add(Card::new("♣","8",6));
	deck.add(Card::new("♣","9",7));
	deck.add(Card::new("♣","10",8));
	deck.add(Card::new("♣","J",9));
	deck.add(Card::new("♣","Q",10));
	deck.add(Card::new("♣","K",11));
	deck.add(Card::new("♣","A",12));
	deck.add(Card::new("♣","2",20));
	
	deck.add(Card::new("♦","3",1));
	deck.add(Card::new("♦","4",2));
	deck.add(Card::new("♦","5",3));
	deck.add(Card::new("♦","6",4));
	deck.add(Card::new("♦","7",5));
	deck.add(Card::new("♦","8",6));
	deck.add(Card::new("♦","9",7));
	deck.add(Card::new("♦","10",8));
	deck.add(Card::new("♦","J",9));
	deck.add(Card::new("♦","Q",10));
	deck.add(Card::new("♦","K",11));
	deck.add(Card::new("♦","A",12));
	deck.add(Card::new("♦","2",20));
	
	deck.add(Card::new("☆","B",30));
	deck.add(Card::new("★","R",40));
	
	deck.shuffle();
	
	deck
}
