let read_file filename =
  In_channel.with_open_text filename In_channel.input_lines

let parse_range range =
  match String.split_on_char '-' range with
  | [ a; b ] -> (int_of_string a, int_of_string b)
  | _ -> invalid_arg "invalid range"

let rec all_equal list =
  match list with
  | a :: b :: r -> a = b && all_equal (b :: r)
  | [] | [_] -> true

let check_subset string size =
  let len = String.length string in
  if len mod size != 0 then false
  else
    let subsets =
      List.init (len / size) (fun i -> String.sub string (i * size) size)
    in
    all_equal subsets

let rec check_all_subsets string mid size =
  if size > mid then false
  else if check_subset string size then true
  else check_all_subsets string mid (size + 1)

let is_invalid a =
  let a_str = string_of_int a in
  let len = String.length a_str in
  check_all_subsets a_str (len / 2) 1

let rec check_range acc (f, t) =
  if f > t then acc
  else
    let new_acc = if is_invalid f then acc + f else acc in
    check_range new_acc (f + 1, t)

let () =
  read_file "input.txt"
  |> List.concat_map (fun l ->
      String.split_on_char ',' l |> List.map parse_range)
  (* multithreaded *)
  |> List.map (fun chunk -> Domain.spawn (fun () -> (check_range 0) chunk))
  |> List.map Domain.join
  (* or single threaded *)
  (* |> List.map (check_range 0) *)
  |> List.fold_left ( + ) 0 
  |> Printf.printf "%d\n"
