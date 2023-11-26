module spiral
    implicit none

    integer, parameter :: max_side = 100000
    integer :: val = 2, side = 0
    integer, dimension(2) :: pos = [1, 0], dir = [0, 1]
    ! diag1: sign(x) == sign(y)
    ! diag2: sign(x) == -sign(y)
    integer, dimension(2, -max_side:max_side) :: diag
    logical, dimension(2, -max_side:max_side) :: primes


contains
    subroutine init()
        diag(:, 0) = 1
        primes(:, :) = .false.
    end subroutine init

    subroutine rotate()
        dir = [-dir(2), dir(1)]
    end subroutine rotate

    subroutine extend()
        integer, dimension(2) :: start_pos
        
        start_pos = pos
        val = val + 1
        pos = pos + dir
        do while (.not. all(pos == start_pos))
            if (pos(1) == pos(2)) then
                call rotate()
                diag(1, pos(1)) = val
                primes(1, pos(1)) = is_prime(val)
                
            else if (pos(1) == -pos(2)) then
                call rotate()
                diag(2, pos(1)) = val
                primes(2, pos(1)) = is_prime(val)
            end if

            val = val + 1
            pos = pos + dir
        
        end do
        
        side = side + 1
        pos = pos - dir + [1, 0]
    end subroutine extend

    elemental function is_prime(x)
        integer, intent(in) :: x
        integer :: i
        logical :: is_prime

        i = 2
        is_prime = .false.
        if (x < 2) then
            return
        end if        
        
        is_prime = .true.
        do while (i ** 2 <= x)
            if (mod(x, i) == 0) then
                is_prime = .false.
                return
            end if
            i = i + 1
        end do
    end function is_prime

    function prime_ratio()
        real :: prime_ratio
        integer :: n

        n = 4 * side + 1
        prime_ratio = sum(merge(1.0, 0.0, primes(:, -side:side))) / n

    end function prime_ratio
end module spiral

program p58
    use spiral

    implicit none

    integer :: i
    real :: ratio

    call init()
    
    do i = 1, max_side
        call extend()
        ratio = prime_ratio()
        if (ratio < 0.1) then
            print *, side * 2 + 1, ratio
            exit
        end if
    end do
    
end program p58
