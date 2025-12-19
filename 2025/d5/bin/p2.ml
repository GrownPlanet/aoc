let read_input filename =
  In_channel.with_open_text filename In_channel.input_lines

let rec split_input acc lines =
  match lines with
  | line :: rest ->
      if line = "" then (acc, rest) else split_input (line :: acc) rest
  | _ -> invalid_arg "no blank line"

type mode = Start | End

let int_of_mode mode = match mode with Start -> 1 | End -> -1

let compare_range (a, m1) (b, m2) =
  let diff = a - b in
  if diff = 0 then int_of_mode m2 - int_of_mode m1 else diff

let parse_range range =
  Scanf.sscanf range "%d-%d" (fun f t -> [ (f, Start); (t, End) ])

let rec filter_ranges acc keep_next ranges =
  match ranges with
  | (v, m) :: r ->
      let new_acc = acc + int_of_mode m in
      if keep_next then v :: filter_ranges new_acc false r
      else if new_acc = 0 then v :: filter_ranges new_acc true r
      else filter_ranges new_acc false r
  | [] -> []

let rec count_fresh values =
  match values with a :: b :: r -> b - a + 1 + count_fresh r | _ -> 0

let () =
  let rranges, _ = read_input "input.txt" |> split_input [] in
  List.concat_map parse_range rranges
  |> List.sort compare_range |> filter_ranges 0 true |> count_fresh
  |> Printf.printf "%d\n"
