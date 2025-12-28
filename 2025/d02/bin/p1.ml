let read_file filename =
  In_channel.with_open_text filename In_channel.input_lines

let parse_range range =
  match String.split_on_char '-' range with
  | [ a; b ] -> (int_of_string a, int_of_string b)
  | _ -> invalid_arg "invalid range"

let is_invalid a =
  let a_str = string_of_int a in
  let len = String.length a_str in
  if len mod 2 <> 0 then false
  else
    let m = len / 2 in
    String.sub a_str 0 m = String.sub a_str m m

let rec check_range acc (f, t) =
  if f > t then acc
  else
    let new_acc = if is_invalid f then acc + f else acc in
    check_range new_acc (f + 1, t)

let () =
  read_file "input.txt"
  |> List.concat_map (fun l ->
      String.split_on_char ',' l |> List.map parse_range)
  |> List.map (check_range 0)
  |> List.fold_left ( + ) 0 |> Printf.printf "%d\n"
