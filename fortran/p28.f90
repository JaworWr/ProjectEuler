program p28

    implicit none
    
    integer, parameter :: N = 1001
    integer, dimension(N, N) :: spiral = 0
    integer, dimension(N) :: diag1, diag2
    integer, dimension(2) :: p = ([N, N] + 1) / 2, p1, d = [0, 1], dt
    integer :: i, val = 1

    dt = turn_right(d)

    do while (is_valid(p))
        spiral(p(1), p(2)) = val
        p1 = p + dt
        
        if (spiral(p1(1), p1(2)) .eq. 0) then
            d = dt
            dt = turn_right(d) 
        end if
        
        p = p + d
        val = val + 1

        ! if (val .gt. 5) then
        !     exit
        ! end if
    end do

    ! print *, p, d, dt
    ! print *
    ! do i = 1, N
    !     print *, spiral(:, N - i + 1)
    ! end do
    
    diag1 = [(spiral(i, i), i = 1, N)]
    diag2 = [(spiral(i, N - i + 1), i = 1, N)]
    print *, sum(diag1) + sum(diag2) - 1

contains
    function turn_right(d) result(r)
        integer, dimension(2), intent(in) :: d
        integer, dimension(2) :: r

        r(1) = d(2)
        r(2) = -d(1)
    end

    function is_valid(d)
        integer, dimension(2), intent(in) :: d
        logical :: is_valid

        is_valid = all(d .le. N .and. d .ge. 1)
    end
end program p28
