open Z3
open Z3.Arithmetic
open Z3.Optimize

let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let parse_button button =
  String.sub button 1 (String.length button - 2)
  |> String.split_on_char ','
  |> List.map (fun n -> int_of_string n)
  |> List.rev

let parse_joltage joltage =
  String.sub joltage 1 (String.length joltage - 2)
  |> String.split_on_char ',' |> List.map int_of_string

let rec parse_split acc split =
  match split with
  | [ j ] -> (List.rev acc, parse_joltage j)
  | button :: r -> parse_split (parse_button button :: acc) r
  | [] -> invalid_arg "invalid machine"

let parse_line line =
  let split = String.split_on_char ' ' line in
  let buttons, joltages = List.tl split |> parse_split [] in
  (buttons, joltages)

let solve (vars_ids, sols) =
  let len = List.length vars_ids in
  let cfg = [ ("model", "true") ] in
  let ctx = mk_context cfg in
  let opt = mk_opt ctx in
  let n =
    Array.init len (fun i -> Integer.mk_const_s ctx ("n" ^ string_of_int i))
  in
  let zero = Integer.mk_numeral_i ctx 0 in
  Array.iter (fun ni -> add opt [ mk_ge ctx ni zero ]) n;
  List.iteri
    (fun i s ->
      let v =
        vars_ids
        |> List.mapi (fun j v -> (j, List.mem i v))
        |> List.filter (fun (_, c) -> c)
        |> List.map (fun (j, _) -> n.(j))
      in
      let sum_expr = mk_add ctx v in
      let target = Integer.mk_numeral_i ctx s in
      add opt [ Boolean.mk_eq ctx sum_expr target ])
    sols;
  let minimize_sum = mk_add ctx (List.init len (fun i -> n.(i))) in
  let _ = minimize opt minimize_sum in
  match check opt with
  | SATISFIABLE -> (
      match get_model opt with
      | Some model ->
          let total =
            Model.evaluate model minimize_sum true
            |> Option.get |> Expr.to_string |> int_of_string
          in
          total
      | None -> invalid_arg "Could not retrieve model.")
  | UNSATISFIABLE -> invalid_arg "No solution exists."
  | UNKNOWN -> invalid_arg "The solver could not determine a solution."

let () =
  read_input "input.txt"
  |> List.map (fun m -> m |> parse_line |> solve)
  |> List.fold_left ( + ) 0 |> Printf.printf "%d\n"
