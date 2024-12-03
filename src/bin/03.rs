use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let line = input.lines();
// @~don't()mul(683,461) >,~select()what()};<mul(848,589)!#{$$:,#mul(597,936)]!how();)+mul(944,148)who()mul(84,922)what()(mul(95,23)[;]*mul(186,673)$@+,;mul(662,571),^^&&mul(467,635)]>what()*,why()mul(456,228)%/'where()~{mul(508,422)mul(78,184)&why()/(mul(373,546)why()*</mul(840,607)(:how(428,64)>why():-~{@mul(615,760)}?/*>$mul(422,670)},?#''how()@#mul(597,259)??mul(281,991)when()why()]/;}:who()why()/mul(864,121)how()#{[from()&mul(336,536) what()/what()#:>mul(908,667)[%^>&]select()*(mul(716,31)what()[+~what())]where():mul(131,416)what()from()]^when()$mul(808,465)from(747,493)*@;mul(728,405)@mul(528,186)where(830,813) mul(138,404)&&$,,mul(220,842)(from()*~from()how()where()mul(44,647)~where()}%$+mul(523,78)}mul(506,925)where()) ?mul(650,260)what()))when()mul(992,831)*]?-(mul(382,514)<?mul(338,811)~who()<)mul(510,851)[mul(362,413)^why()how()mul(456,506)why())},)*where(294,226)do()<when()>@<>,{*<mul(695,394)from(){/~mul(192,894);how()where()what()who()!:mul(932,681)@select()(where()@{?^+mul(260,501)from()+select()?^mul(445,400)@:;>^/why()[don't()#>how(558,115)mul(303,593)${~from()%*$%do()-+[$mul(406,365)how()mul(561,819)how():select()[:when()mul(283,930)where()$>why():<from(){who()^mul(5,405);%#what()[(*>#mul(154,725)from(496,796))@[how()from()- mul(942,491)-don't()~who()^)#select()where()mul(880,650)>(when(),)who()]where()mul(735,764)<-select()where()~#mul(685,917)from()!how()&who(),~why()[/mul(838,662) }what()from()from()<mul(25,821)]$+?)[>[mul(217,645)who(){who()mul(712,250)*}~who(133,126)where()}what()-where()mul(879,905)mul(95,8)~{?%:/mul(885,299)mul(509,215)~mul(97,351)mul(11,228)mul(976,139)where()mul(532,340)mul(535,236)</@)+mul(91,686)^+)(/}where()why()(<mul#-+!%(mul(23,448)!{:}#,mul(761,358)select()<mul(557,974)<({select()& why()]'mul(940,978)from()*why(871,616)$[mul#mul(652,248)what(455,217)-$<)'!{who(),mul(530,568))<;[}^do(){where()#&$};,}mul(903,867);-#**[/~mul(698,27),~mul(701,325)where()#who()>)'mul(544,423)@@ mul(543,903)why()mul(610,22)where()when()^)mul(529,383) ;,/'when(179,924)mul(31,223)when());<mul(812,214)mul(375,504)]:(~& !who()[~mul(192,595){~[{what()who()'from()mul(127,309)who()mul(555,543)%/%!~select(),?[from()mul(832,869)mul(808,878){[@mul(301,808)?>*from()mul(810,928) ^'{'?~mul(840,456)?where()}~:who()'do()@where(){(@$select(471,179)mul(490,95)where()?>)<mul(355,525)from(971,891)+select(){mul(247,601))%how()[;~select()mul(960,14)? #}/~from()why()%from()mul(507,980+from()#@@mul(50,394)how(751,319)who()?~what()'when()$}-mul(791,444)))!#/>^$mul(192,331)?when()where()!}who()*%[%mul(778,800)+}when()who()how()mul(836,264)from()+)@where()mul(261,535)&)@why()-#/(mul(187,2)(?what()mul(467,281)when()where()#, [+what()select()(mul(232,619)don't()^mul(406,760)mul(400,865)%%who()where()}${?[!don't()#%@^/)when()]@%mul(268,776){-:how()}:'~+mul(817,838)}when()when()#/where()?how()+mul(712,30)select()(%* !mul(11,366)

    let re = Regex::new(r"mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();
    let mut ret = 0;
    for l in line {
        let caps: Vec<u32> = re.captures_iter(l).map(|m| {
            let num1: u32 = m["num1"].parse::<u32>().unwrap();
            let num2: u32 = m["num2"].parse::<u32>().unwrap();
            num1 * num2
        }).collect();

        for cap in caps {
            println!("{}", cap);
            ret += cap;
        }

    }


    return Some(ret);
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
