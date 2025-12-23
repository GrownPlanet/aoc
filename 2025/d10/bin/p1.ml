let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

type machine = { target : int; state : int; buttons : int list }

module MachineSet = Set.Make (struct
  type t = machine

  let compare m1 m2 = m1.state - m2.state
end)

let parse_button button =
  String.sub button 1 (String.length button - 2)
  |> String.split_on_char ','
  |> List.map (fun n -> Int.shift_left 1 (int_of_string n))
  |> List.fold_left Int.logor 0

let rec parse_split split =
  match split with
  | [] | [ _ ] -> []
  | button :: r -> parse_button button :: parse_split r

let parse_target target =
  let targ =
    String.sub target 1 (String.length target - 2)
    |> String.map (fun ch -> if ch = '.' then '0' else '1')
  in
  let len = String.length targ in
  let rev = String.init len (fun i -> String.get targ (len - i - 1)) in
  "0b" ^ rev |> int_of_string

let parse_line line =
  let split = String.split_on_char ' ' line in
  let target = List.hd split |> parse_target in
  let buttons = List.tl split |> parse_split in
  { target; state = 0; buttons }

let press_buttons machine =
  List.map
    (fun b -> { machine with state = Int.logxor machine.state b })
    machine.buttons

let rec solve_machines i mem machines =
  let l =
    List.concat_map press_buttons machines
    |> List.filter (fun m -> MachineSet.find_opt m mem = None)
  in
  let new_seen = MachineSet.union (MachineSet.of_list l) mem in
  match List.find_opt (fun m -> m.state = m.target) l with
  | Some _ -> i
  | None -> solve_machines (i + 1) new_seen l

let () =
  read_input "input.txt"
  |> List.map (fun m -> solve_machines 1 MachineSet.empty [ parse_line m ])
  |> List.fold_left ( + ) 0 |> Printf.printf "%d\n"
