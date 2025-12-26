let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let parse_button button =
  String.sub button 1 (String.length button - 2)
  |> String.split_on_char ','
  |> List.map (fun n -> int_of_string n)
  |> List.rev

let parse_joltage joltage =
  String.sub joltage 1 (String.length joltage - 2)
  |> String.split_on_char ',' |> List.map float_of_string |> Array.of_list

let rec parse_split acc split =
  match split with
  | [ j ] -> (List.rev acc, parse_joltage j)
  | button :: r -> parse_split (parse_button button :: acc) r
  | [] -> invalid_arg "invalid machine"

(*
   n1  n2  ..  Y1  Y2 ..   sol
Z          z             | z_solution
  -----------------------+----------
Y1                       | 
Y2      variables        | solutions
..                       | 
*)

type simplex_alg = {
  z : float array;
  z_solution : float;
  solutions : float array;
  variables : float array array;
}

let create_simplex variables solutions =
  let vc = List.length variables in
  let sc = Array.length solutions in
  let get_cell_value r c =
    if c < vc then if List.mem r (List.nth variables c) then 1. else 0.
    else if r = c - vc then 1.
    else 0.
  in
  let variables =
    Array.init_matrix sc (vc * 2) (fun r c -> get_cell_value r c)
  in
  let z =
    Array.fold_left (Array.map2 ( +. ))
      (Array.init (vc * 2) (fun c -> if c >= vc then -1. else 0.))
      variables
  in
  let z_solution = Array.fold_left ( +. ) 0. solutions in
  { z; z_solution; solutions; variables }

let parse_line line =
  let split = String.split_on_char ' ' line in
  let buttons, joltages = List.tl split |> parse_split [] in
  let s = create_simplex buttons joltages in
  s

let rec max_indecies mi ci arr =
  if ci >= Array.length arr then mi
  else if arr.(ci) > arr.(List.hd mi) then max_indecies [ci] (ci + 1) arr
  else if arr.(ci) = arr.(List.hd mi) then max_indecies (ci :: mi) (ci + 1) arr
  else max_indecies mi (ci + 1) arr

let safe_div a b = if b = 0. then None else Some (a /. b)

let rec smallest_ratio mi ci mr pos_idx simplex =
  if ci >= Array.length simplex.solutions then mi
  else
    let var = simplex.variables.(ci).(pos_idx) in
    let sol = simplex.solutions.(ci) in
    if var > 0. then
      match safe_div sol var with
      | Some x when 0. <= x && x < mr ->
          smallest_ratio ci (ci + 1) x pos_idx simplex
      | _ -> smallest_ratio mi (ci + 1) mr pos_idx simplex
    else smallest_ratio mi (ci + 1) mr pos_idx simplex

let rec first_valid_pivot mpos_ids simplex =
  match mpos_ids with
  | mpos_idx :: r ->
    let sratio_idx = smallest_ratio (-1) 0 Float.infinity mpos_idx simplex in
    if sratio_idx = -1 then first_valid_pivot r simplex else Some (mpos_idx, sratio_idx)
  | [] -> None

let find_pivot simplex =
  let mpos_ids = max_indecies [0] 0 simplex.z in
  if simplex.z.(List.hd mpos_ids) <= 0. then None
  else
    first_valid_pivot mpos_ids simplex

let devide_row row_idx by simplex =
  simplex.variables.(row_idx) <-
    Array.map (fun a -> a /. by) simplex.variables.(row_idx);
  simplex.solutions.(row_idx) <- simplex.solutions.(row_idx) /. by

let subtract_rows i1 i2 mult simplex =
  simplex.variables.(i1) <-
    Array.map2
      (fun a b -> a -. (b *. mult))
      simplex.variables.(i1) simplex.variables.(i2);
  simplex.solutions.(i1) <-
    simplex.solutions.(i1) -. (simplex.solutions.(i2) *. mult)

let develop r c simplex =
  devide_row c simplex.variables.(c).(r) simplex;
  (* TODO: add z & z_solution to the rest of the data?? *)
  for i = 0 to c - 1 do
    subtract_rows i c simplex.variables.(i).(r) simplex
  done;
  for i = c + 1 to Array.length simplex.variables - 1 do
    subtract_rows i c simplex.variables.(i).(r) simplex
  done;
  let mult = simplex.z.(r) in
  let newz =
    Array.map2 (fun a b -> a -. (b *. mult)) simplex.z simplex.variables.(c)
  in
  let newz_sol = simplex.z_solution -. (simplex.solutions.(c) *. mult) in
  { simplex with z = newz; z_solution = newz_sol }

let rec solve_simplex simplex =
  let piv = find_pivot simplex in
  match piv with
  | Some (pr, pc) ->
      let simplex = develop pr pc simplex in
      solve_simplex simplex
  | None ->
    Array.iter (Printf.printf "%f ") simplex.solutions;
    let s = Array.fold_left ( +. ) 0. simplex.solutions in
    Printf.printf "= %f -> %d" s (int_of_float s); print_newline ();
    int_of_float s

let () =
  read_input "test_input.txt"
  |> List.map (fun m -> parse_line m |> solve_simplex)
  |> List.fold_left ( + ) 0 |> Printf.printf "%d\n"
