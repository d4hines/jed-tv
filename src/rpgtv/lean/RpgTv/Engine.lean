import Rpg.Play

/- The original pure game runs in the app process. Native code only draws
the view and sends a selected option index; it cannot invent game inputs. -/
namespace RpgTv
open Rpg

structure Session where
  game : Game
  dice : Rng
  seed : UInt32
  reports : List Report := []

@[export tv_rpg_new]
def start (seed : UInt32) : Session :=
  {game := Game.new Lighthouse.lighthouseDefinition, dice := ⟨seed.toUInt64⟩, seed}

@[export tv_rpg_choose]
def choose (s : Session) (index : UInt32) : Session :=
  match s.game.phase with
  | .ended _ | .dead _ => if index == 0 then start (s.seed + 1) else s
  | _ => match s.game.inputs[index.toNat]? with
    | none => s
    | some input =>
      let ((game, reports), dice) := (Rpg.step s.game input).run s.dice
      {s with game, dice, reports := s.reports ++ reports}

@[extern "tv_rpg_header"] opaque header (phase adventure heading body picture : @& String) (mood : UInt32) : IO Unit
@[extern "tv_rpg_bar"] opaque bar (kind : UInt32) (name : @& String) (hp : Int32) (maximum alive : UInt32) : IO Unit
@[extern "tv_rpg_side_title"] opaque sideTitle (value : @& String) : IO Unit
@[extern "tv_rpg_side"] opaque side (value : @& String) : IO Unit
@[extern "tv_rpg_option"] opaque option (label help : @& String) : IO Unit

private def rollText (r : DiceRoll) : String :=
  let result := match r.result with
    | .success => "success" | .failure => "failure"
    | .criticalSuccess => "critical success" | .criticalFailure => "critical failure"
  String.intercalate "+" (r.faces.map toString) ++ s!" = {r.total} vs {r.target}: {result}"

private def maneuverLabel : Maneuver → String
  | .attack => "Attack" | .allOutAttack => "All-Out Attack" | .allOutDefense => "All-Out Defense"
private def maneuverHelp : Maneuver → String
  | .attack => "Attack, and Dodge as usual."
  | .allOutAttack => "+4 to hit, but no Dodge until your next turn."
  | .allOutDefense => "No attack, but +2 to Dodge until your next turn."

private def reportText : Report → Option String
  | .scene .. => none
  | .picked text => some ("You chose: " ++ text)
  | .successRoll who target modifier roll => some s!"{who} rolls {target.name} {modifier}: {rollText roll}"
  | .cannotTry who target .. => some s!"{who} cannot try {target.name}."
  | .fightStarts hero foes => some s!"A fight starts: {hero.name} against {String.intercalate ", " (foes.map (·.name))}."
  | .round n => some s!"Round {n}."
  | .maneuver who m => some s!"{who} chooses {maneuverLabel m}."
  | .attack _ who _ target weapon roll => some s!"{who} attacks {target} with {weapon.name}: {rollText roll}"
  | .cannotAttack who _ => some s!"{who} cannot attack."
  | .noDefense who => some s!"{who} cannot defend."
  | .dodge who roll => some s!"{who} dodges: {rollText roll}"
  | .damage _ who _ _ _ _ _ _ injury hp => some s!"{who} takes {injury} injury ({hp} HP left)."
  | .falls _ name => some s!"{name} falls."
  | .fightWon => some "The fight is won."
  | .died who => some s!"{who} dies."
  | .rejected why => some ("Refused: " ++ why)

@[export tv_rpg_render]
def render (s : Session) : IO Unit := do
  let g := s.game
  let scene := g.sceneId?.bind g.adventure.scene?
  let title := (scene.map (·.title)).getD g.adventure.title
  let text := (scene.map (·.text)).getD "A cold fishing coast, where storms roll in off the sea and wreckers wait for ships to break on the reef. You are Mara, a sellsword who works for coin, with a broadsword, leather armor, and quick hands for locks."
  let picture := (scene.bind (·.picture)).filter safePicture |>.getD ""
  let (heading, mood) := match g.phase with
    | .dead _ => (g.hero.name ++ " has died", 1)
    | .ended _ => ("The End: " ++ title, 2)
    | .fight f => (title ++ s!" - round {f.round}", 0)
    | _ => (title, 0)
  header g.phase.name g.adventure.title heading text picture mood.toUInt32
  let hp : Int := match g.phase with | .fight f | .dead f => f.state.hero.hp | _ => g.hp
  bar 0 g.hero.name hp.toInt32 (g.rules.maxHP g.hero).toUInt32 (if hp > 0 then 1 else 0)
  match g.phase with
  | .title =>
    sideTitle "Your character"
    side s!"{g.hero.name}: ST {g.hero.st}  DX {g.hero.dx}  IQ {g.hero.iq}  HT {g.hero.ht}"
    side s!"HP {g.rules.maxHP g.hero}  Dodge {g.rules.dodge g.hero}"
    side s!"{g.adventure.themes.weapon g.hero.weapon} ({(g.rules.damage g.hero.weapon g.hero.st).show} {g.hero.weapon.damageType.name}), {g.adventure.themes.armorName g.hero.armor}"
    for (skill, level) in g.hero.skills do side s!"{g.adventure.themes.skill skill} {level}"
    side "A sellsword who has guarded caravans across three kingdoms, and opened more than one door that was meant to stay shut."
  | _ =>
    sideTitle "What happened"
    let reports := s.reports.filterMap reportText
    for text in reports.drop (reports.length - 12) do side text
  match g.phase with
  | .fight f | .dead f =>
    for foe in f.state.foes do
      bar 1 foe.being.name foe.hp.toInt32 (g.rules.maxHP foe.being).toUInt32 (if foe.alive then 1 else 0)
  | _ => pure ()
  for input in g.inputs do
    match input with
    | .begin => option "Begin the adventure" "Go to the first scene."
    | .choose k =>
      if let some choice := scene.bind (fun scene => scene.choices[k]?) then option choice.text ""
    | .maneuver maneuver target =>
      let foe := match g.phase with
        | .fight f => target.bind (fun k => f.state.foes[k]?) | _ => none
      let label := maneuverLabel maneuver ++ (foe.map (fun f => " " ++ f.being.name)).getD ""
      option label (maneuverHelp maneuver)
  match g.phase with
  | .ended _ => option "Play again" "Start this adventure again, with new dice."
  | .dead _ => option "Try again" "Start this adventure again, with new dice."
  | _ => pure ()

@[export tv_rpg_io_unit] def ioUnit : IO Unit := pure ()

end RpgTv
