type condition =
  | If of { variable : char; operation : char; num : int; then_target : string }
  | Else of string

type params = { x : int * int; m : int * int; a : int * int; s : int * int }

let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let rec take_first acc lines =
  match lines with
  | [] | "" :: _ -> List.rev acc
  | l :: r -> take_first (l :: acc) r

let parse_condition c =
  if String.length c < 2 || (String.get c 1 <> '>' && String.get c 1 <> '<')
  then Else c
  else
    let variable = String.get c 0 in
    let operation = String.get c 1 in
    let rest =
      String.sub c 2 (String.length c - 2) |> String.split_on_char ':'
    in
    match rest with
    | [ num; then_target ] ->
        If { variable; operation; num = int_of_string num; then_target }
    | _ -> raise (Failure "wrong rest")

let parse_line l =
  let name_end = String.index l '{' in
  let name = String.sub l 0 name_end in
  let conditions =
    String.sub l (name_end + 1) (String.length l - name_end - 2)
    |> String.split_on_char ',' |> List.map parse_condition
  in
  (name, conditions)

let get_param name param =
  match name with
  | 'x' -> param.x
  | 'm' -> param.m
  | 'a' -> param.a
  | 's' -> param.s
  | _ -> raise (Failure "wrong param")

let count_params params =
  [ params.x; params.m; params.a; params.s ]
  |> List.fold_left (fun a (s, e) -> a * (e - s + 1)) 1

let cutoff_variable params var operation num =
  let s, e = get_param var params in
  let if_range, else_range =
    match operation with
    | '<' -> ((s, num - 1), (num, e))
    | '>' -> ((num + 1, e), (s, num))
    | _ -> raise (Failure "wrong oper")
  in
  match var with
  | 'x' -> ({ params with x = if_range }, { params with x = else_range })
  | 'm' -> ({ params with m = if_range }, { params with m = else_range })
  | 'a' -> ({ params with a = if_range }, { params with a = else_range })
  | 's' -> ({ params with s = if_range }, { params with s = else_range })
  | _ -> raise (Failure "wrong param")

let rec count_accepts params name map =
  if name = "A" then count_params params
  else if name = "R" then 0
  else
    let conds = Hashtbl.find map name in
    check_conds params conds map

and check_conds params conds map =
  match conds with
  | [] -> 0
  | cond :: rest -> (
      match cond with
      | Else c -> count_accepts params c map
      | If c ->
          let if_params, else_params =
            cutoff_variable params c.variable c.operation c.num
          in
          count_accepts if_params c.then_target map
          + check_conds else_params rest map)

let () =
  let raw_input = read_input "input.txt" |> take_first [] in
  let map = Hashtbl.create (List.length raw_input) in
  List.iter
    (fun s ->
      let k, v = parse_line s in
      Hashtbl.add map k v)
    raw_input;
  let res =
    count_accepts
      { x = (1, 4000); m = (1, 4000); a = (1, 4000); s = (1, 4000) }
      "in" map
  in
  Printf.printf "%d\n" res
