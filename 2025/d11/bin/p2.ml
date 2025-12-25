let read_input filename = In_channel.(with_open_text filename input_lines)

let parse_line line =
  match String.split_on_char ':' line with
  | [ f; t ] -> (f, String.split_on_char ' ' t |> List.filter (fun s -> s <> ""))
  | _ -> invalid_arg "invalid input line"

let parse tbl lines =
  List.iter
    (fun l ->
      let f, t = parse_line l in
      Hashtbl.add tbl f t)
    lines;
  tbl

type count = { dacfft : int; dac : int; fft : int; none : int }

let add_counts c n =
  {
    none = c.none + n.none;
    dac = c.dac + n.dac;
    fft = c.fft + n.fft;
    dacfft = c.dacfft + n.dacfft;
  }

let dac_count c = { dacfft = c.fft; dac = c.none; none = 0; fft = 0 }
let fft_count c = { dacfft = c.dac; fft = c.none; none = 0; dac = 0 }

let rec count from mem conmap =
  if from = "out" then { none = 1; fft = 0; dac = 0; dacfft = 0 }
  else
    match Hashtbl.find_opt mem from with
    | Some x -> x
    | None ->
        let new_count =
          List.fold_left
            (fun acc con -> add_counts acc (count con mem conmap))
            { dacfft = 0; dac = 0; fft = 0; none = 0 }
            (Hashtbl.find conmap from)
        in
        let new_count =
          match from with
          | "dac" -> dac_count new_count
          | "fft" -> fft_count new_count
          | _ -> new_count
        in
        Hashtbl.add mem from new_count;
        new_count

let () =
  let n =
    read_input "input.txt"
    |> parse (Hashtbl.create 512)
    |> count "svr" (Hashtbl.create 512)
  in
  Printf.printf "%d\n" n.dacfft
